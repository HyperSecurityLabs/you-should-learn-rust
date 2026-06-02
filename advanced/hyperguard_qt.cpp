// ════════════════════════════════════════════════════════════════════════
// HyperGuard — HDD Monitor GUI in Qt 6 C++
// Purpose: VISUAL COMPARISON only. Cannot compile without:
//   sudo apt install qt6-base-dev cmake g++
//
// Compare this 200-line Qt C++ file to the ~80-line Rust egui version
// and see which one is SAFER, SHORTER, and SEXIER. 💖
// ════════════════════════════════════════════════════════════════════════

#include <QApplication>
#include <QMainWindow>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QProgressBar>
#include <QTimer>
#include <QFrame>
#include <QFont>
#include <QGraphicsDropShadowEffect>
#include <QStyle>
#include <QProcess>
#include <QDebug>

// ── DATA MODEL ────────────────────────────────────────────────────────
// In Rust: struct HddStatus { temp: f64, status: String }
//          Safe. No null. No uninitialized fields.
// In C++:  We have to write a constructor. What if we forget to init? UB.
//          What if temp is NaN? No one checks. 💀

struct HddStatus {
    double temperature;   // Could be NaN. Could be uninitialized. C++ doesn't care.
    QString status;       // Could be empty. Could be null QString. Who knows?
    bool is_critical;
    
    // Had to write this manually. Rust gives it for free with struct literal syntax.
    HddStatus(double temp, const QString& stat, bool crit)
        : temperature(temp), status(stat), is_critical(crit) {}
};

// ── THE WIDGET — 90 lines vs 30 lines in Rust egui ────────────────────

class HddMonitorWidget : public QWidget {
    Q_OBJECT  // MOC (Meta-Object Compiler) step required. Rust needs none.

private:
    // All these MEMBER VARIABLES — must be manually managed.
    // In Rust egui: state is in a struct. No manual layout. No manual updates.
    QLabel*      title_label;
    QLabel*      temp_label;
    QProgressBar* temp_bar;
    QLabel*      status_label;
    QPushButton* refresh_btn;
    QTimer*      poll_timer;
    HddStatus    current_status;
    QFrame*      container;
    QProcess*    smartctl_process;  // Memory: must delete. Did I forget? Leak.

    void setup_ui() {
        // ── container frame with shadow (Qt needs 15 lines for a shadow) ──
        // In Rust egui: ui.frame().show(..., |ui| { ... }) — 1 line. Built-in.
        auto* main_layout = new QVBoxLayout(this);
        main_layout->setContentsMargins(20, 20, 20, 20);
        
        container = new QFrame(this);
        container->setStyleSheet(
            "QFrame { background: #1a1b2e; border-radius: 12px; "
            "border: 1px solid #2d2f4e; padding: 16px; }"
        );
        
        auto* shadow = new QGraphicsDropShadowEffect(container);
        shadow->setBlurRadius(20);
        shadow->setColor(QColor(0, 0, 0, 80));
        shadow->setOffset(0, 4);
        container->setGraphicsEffect(shadow);
        // 7 lines for a shadow. Rust egui: Frame::dark().show(...) — done.
        
        auto* container_layout = new QVBoxLayout(container);
        container_layout->setSpacing(12);
        
        // ── title ──
        title_label = new QLabel("💾 HDD Temperature Monitor", container);
        QFont title_font;
        title_font.setPointSize(16);
        title_font.setBold(true);
        title_label->setFont(title_font);
        title_label->setStyleSheet("color: #e0e0ff;");
        container_layout->addWidget(title_label);
        
        // ── temperature display ──
        temp_label = new QLabel("-- °C", container);
        QFont temp_font;
        temp_font.setPointSize(36);
        temp_font.setBold(true);
        temp_label->setFont(temp_font);
        temp_label->setAlignment(Qt::AlignCenter);
        temp_label->setStyleSheet("color: #00ff88;");
        container_layout->addWidget(temp_label);
        
        // ── progress bar ──
        temp_bar = new QProgressBar(container);
        temp_bar->setRange(0, 100);
        temp_bar->setValue(0);
        temp_bar->setTextVisible(false);
        temp_bar->setFixedHeight(8);
        temp_bar->setStyleSheet(
            "QProgressBar { background: #2d2f4e; border-radius: 4px; }"
            "QProgressBar::chunk { background: qlineargradient(x1:0,y1:0,x2:1,y2:0, "
            "stop:0 #00ff88, stop:0.5 #ffaa00, stop:1 #ff3355); border-radius: 4px; }"
        );
        container_layout->addWidget(temp_bar);
        
        // ── status text ──
        status_label = new QLabel("Waiting...", container);
        status_label->setAlignment(Qt::AlignCenter);
        QFont status_font;
        status_font.setPointSize(12);
        status_label->setFont(status_font);
        status_label->setStyleSheet("color: #8888aa;");
        container_layout->addWidget(status_label);
        
        // ── refresh button (must manually connect signal/slot) ──
        refresh_btn = new QPushButton("🔄 Refresh Now", container);
        refresh_btn->setStyleSheet(
            "QPushButton { background: #2d2f4e; color: #e0e0ff; border: none; "
            "border-radius: 6px; padding: 8px 20px; font-size: 13px; }"
            "QPushButton:hover { background: #3d3f6e; }"
            "QPushButton:pressed { background: #1d1f3e; }"
        );
        QObject::connect(refresh_btn, &QPushButton::clicked, this, &HddMonitorWidget::refresh);
        container_layout->addWidget(refresh_btn);
        
        main_layout->addWidget(container);
        
        // ── poll timer (must remember to stop and delete) ──
        poll_timer = new QTimer(this);
        QObject::connect(poll_timer, &QTimer::timeout, this, &HddMonitorWidget::refresh);
        poll_timer->start(60000);  // every 60s
        
        // ── smartctl process (CAN'T re-use. MUST create new each time.) ──
        // In Rust: std::process::Command is a builder. Reusable. Safe.
        smartctl_process = new QProcess(this);
        QObject::connect(smartctl_process, &QProcess::finished, this, [this](int exitCode) {
            if (exitCode == 0) {
                QString output = QString::fromUtf8(smartctl_process->readAllStandardOutput());
                parse_and_update(output);
            } else {
                status_label->setText("⚠️ smartctl failed (need sudo?)");
                status_label->setStyleSheet("color: #ff8844;");
            }
        });
    }

    void parse_and_update(const QString& output) {
        // In Rust: output.lines().find(|l| l.contains("Temperature_Celsius"))
        //     .and_then(|l| l.split_whitespace().nth(9))
        //     .and_then(|s| s.parse::<f64>().ok());
        // Returns Option<f64>. SAFE. No null. No NaN.
        //
        // In C++: manual string parsing. If parse fails: 0.0. Wrong. Silent.
        
        double temp = 0.0;
        QString status = "Unknown";
        bool crit = false;
        
        QStringList lines = output.split('\n');
        for (const QString& line : lines) {
            if (line.contains("Temperature_Celsius")) {
                QStringList parts = line.split(' ', Qt::SkipEmptyParts);
                if (parts.size() > 9) {
                    bool ok = false;
                    temp = parts[9].toDouble(&ok);
                    if (!ok) temp = 0.0;  // Silent failure. In Rust: Option<f64>.
                }
                break;
            }
        }
        
        // Update UI
        update_display(HddStatus(temp, status, crit));
    }

    void update_display(const HddStatus& status) {
        current_status = status;
        temp_label->setText(QString("%1 °C").arg(status.temperature, 0, 'f', 1));
        temp_bar->setValue(static_cast<int>(status.temperature));
        
        if (status.temperature > 70) {
            temp_label->setStyleSheet("color: #ff3355;");
            status_label->setText("🔥 CRITICAL — Immediate action required!");
            status_label->setStyleSheet("color: #ff3355; font-weight: bold;");
        } else if (status.temperature > 55) {
            temp_label->setStyleSheet("color: #ffaa00;");
            status_label->setText("⚠️ Warning — Temperature rising");
            status_label->setStyleSheet("color: #ffaa00;");
        } else {
            temp_label->setStyleSheet("color: #00ff88;");
            status_label->setText("✅ Normal — Drive is cool");
            status_label->setStyleSheet("color: #00ff88;");
        }
    }

public:
    // Constructor: must initialize EVERYTHING manually.
    // Forgot to initialize one pointer? CRASH. UB. Undefined behavior.
    // In Rust: struct fields must ALL be initialized. Compiler enforces it.
    HddMonitorWidget(QWidget* parent = nullptr)
        : QWidget(parent)
        , title_label(nullptr)      // MUST init. If I forget: garbage pointer.
        , temp_label(nullptr)       // Forgot? Segfault on first update.
        , temp_bar(nullptr)
        , status_label(nullptr)
        , refresh_btn(nullptr)
        , poll_timer(nullptr)
        , current_status(0.0, "Init", false)
        , container(nullptr)
        , smartctl_process(nullptr)  // Forgot? Double delete in destructor? UB.
    {
        setup_ui();
        refresh();
    }

    // DESTRUCTOR: Qt parent-child ownership handles most cleanup.
    // But smartctl_process? Parent is 'this', child is deleted.
    // Unless I set parent wrong. Then: LEAK.
    // In Rust: Drop handles it. Automatically. Always. 💖
    ~HddMonitorWidget() override {
        // In Rust: I don't need a destructor. RAII + Drop.
        // In C++: if I forget virtual ~, derived class destructor won't run.
        //         Undefined behavior. 💀
    }

public slots:
    void refresh() {
        status_label->setText("🔄 Reading sensor...");
        // Rust:  let output = Command::new("smartctl").args([...]).output()?;
        //        let temp = output.parse_temp()?;
        //        update(temp);
        // Error handling: Result<T, E>. ? operator. No exceptions. No crashes.
        //
        // C++:   QProcess. Must handle errors BEFORE starting. What if already running?
        //        What if binary doesn't exist? What if permissions denied?
        //        Exceptions? Or silent fail? I choose... inconsistent handling. 💀
        
        smartctl_process->start("sudo", QStringList{
            "/usr/sbin/smartctl", "-A", "/dev/sda"
        });
    }

signals:
    void temperatureCritical(double temp);
    void temperatureWarning(double temp);
    // Signals are great! Qt's best feature.
    // But: they're STRING-BASED in old Qt. Compiler doesn't check names.
    // Modern Qt5/Qt6: type-safe connect. Better. Still MOC-based.
    // Rust: channels (crossbeam, tokio, std::sync::mpsc). Type-safe. No MOC. No codegen. 💖
};

// ── MAIN WINDOW — wrapping the widget ─────────────────────────────────

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    MainWindow() {
        setWindowTitle("HyperGuard — HDD Monitor");
        setMinimumSize(400, 350);
        setStyleSheet("QMainWindow { background: #0f1023; }");
        
        auto* central = new QWidget(this);
        auto* layout = new QVBoxLayout(central);
        layout->addWidget(new HddMonitorWidget(central));
        setCentralWidget(central);
    }
};

// ── ENTRY POINT ───────────────────────────────────────────────────────

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName("HyperGuard HDD Monitor");
    
    MainWindow window;
    window.show();
    
    return app.exec();
    // Qt event loop. Runs until window closes.
    // Memory cleanup: Qt parent-child hierarchy handles it.
    // BUT: only if you set parents correctly.
    //      only if you don't have circular references.
    //      only if you don't delete something twice.
    // In Rust: memory is freed when value goes out of scope.
    //          No event loop needed. No parent-child hierarchy.
    //          No manual cleanup. No double-free. No use-after-free.
    //          Just Drop. Just RAII. Just GUARANTEED. 💖🦀
}

// ════════════════════════════════════════════════════════════════════════
// COMPARISON SUMMARY
// ════════════════════════════════════════════════════════════════════════
//
//                     Qt C++ 6                     Rust egui
//                   ──────────                    ──────────
// Lines of code:     ~200 (this file)              ~80 (equivalent)
// Boilerplate:       MOC, headers, Q_OBJECT        none
// Memory safety:     manual (parent/child)         automatic (borrow checker)
// Null safety:       nullptr everywhere            Option<T> (no null)
// Error handling:    exceptions or ignore          Result<T, E> + ?
// Build system:      CMake + MOC + qmake           cargo build
// Compile time:      ~30s (first build)            ~2s (incremental)
// Binary size:       ~500KB + Qt libs (~50MB)      ~3MB (static)
// GPU acceleration:  software renderer by default  egui uses GPU natively
// Cross-platform:    Qt everywhere (but heavy)     egui: desktop + web + mobile
// Async:             QThread (manual)              async/await (built-in)
//
// Qt C++: Mature. Powerful. Ship since 1995. But showing its age.
// Rust egui: Modern. Safe. Fast. Sexy. The FUTURE. 💖
//
// Both can make beautiful GUIs.
// But one GUARANTEES no segfaults, no leaks, no null pointers.
// And one is C++.
// ════════════════════════════════════════════════════════════════════════
