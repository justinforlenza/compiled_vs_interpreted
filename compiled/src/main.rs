use cpp_core::{Ptr, StaticUpcast};

use qt_core::{qs, slot, QBox, QObject, SlotNoArgs};
use qt_gui::QIntValidator;
use qt_widgets::{
    QApplication, QLineEdit, QPushButton, QWidget, QLabel, QTextEdit
};

use std::time::Instant;
use std::rc::Rc;

struct Application {
    widget: QBox<QWidget>,
    textbox: QBox<QLineEdit>,
    btn: QBox<QPushButton>,
    results: QBox<QTextEdit>
}

impl StaticUpcast<QObject> for Application {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Application {
    fn new() -> Rc<Application> {
        unsafe {
            let widget = QWidget::new_0a();
            widget.resize_2a(300, 300);
            widget.set_fixed_size_2a(300, 300);
            widget.set_window_title(&qs("Prime Number Generator"));

            let textbox = QLineEdit::new();
            let validator = QIntValidator::new_3a(1, 9999999, &textbox);
            textbox.set_validator(&validator);
            textbox.set_parent_1a(&widget);
            textbox.move_2a(20, 20);
            textbox.resize_2a(260, 35);


            let btn = QPushButton::new();
            btn.set_parent_1a(&widget);
            btn.move_2a(20, 60);
            btn.resize_2a(260, 35);
            btn.set_text(&qs("Generate"));
            btn.set_disabled(true);

            let results_label = QLabel::new();
            results_label.set_parent_1a(&widget);
            results_label.move_2a(20, 100);
            results_label.set_text(&qs("Results:"));

            let results = QTextEdit::new();
            results.set_parent_1a(&widget);
            results.set_font_family(&qs("Monospace"));
            results.move_2a(20, 120);
            results.resize_2a(260, 160);
            results.set_read_only(true);

            widget.show();

            let this = Rc::new(Self {
                widget,
                textbox,
                btn,
                results
            });

            this.init();

            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.textbox.text_edited().connect(&self.slot_on_text_changed());

        self.btn.clicked().connect(&self.slot_on_btn_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_text_changed(self: &Rc<Self>) {
        self.btn.set_disabled(self.textbox.text().to_int_0a() < 1);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_btn_clicked(self: &Rc<Self>) {
        let start = Instant::now();

        let primes = self.sieve_of_atkin(self.textbox.text().to_int_0a());
        let primes_count = primes.len();

        let elapsed = start.elapsed().as_millis();

        let mut result_text = vec![
            format!("Time         : {elapsed}ms"),
            format!("Total Found  : {primes_count}"),
            "".to_string(),
            "Numbers:".to_string()
        ];

        for elem in primes.iter() {
            result_text.push(format!("{elem}"))
        }

        self.results.set_text(&qs(result_text.join("\n")))
    }

    fn sieve_of_atkin(self: &Rc<Self>, limit: i32) -> Vec<i32> {
        let mut primes = vec![];

        if limit >= 2 {
            primes.push(2)
        }
        if limit >= 3 {
            primes.push(3)
        }
        if limit == 5 {
            primes.push(5)
        }

        if limit <= 5 {
            return primes
        }

        let mut sieve = vec![false; limit as usize];

        for i in 0..limit as usize {
            sieve[i] = false
        }

        let mut x = 1;

        while x * x < limit {
            let mut y = 1;
            while y * y < limit {
                let mut n = (4 * x * x) + (y * y);

                if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                    sieve[n as usize] ^= true
                }

                n = (3 * x * x) + (y * y);
                if n <= limit && n % 12 == 7 {
                    sieve[n as usize] ^= true
                }

                n = (3 * x * x) - (y * y);
                if x > y && n <= limit && n % 12 == 11 {
                    sieve[n as usize] ^= true
                }

                y += 1
            }
            x += 1
        }

        let mut r = 5;
        while r*r < limit {
            if sieve[r as usize] {
                for i in (r*r..limit).step_by((r*r) as usize) {
                    sieve[i as usize] = false
                }
            }

            r += 1
        }

        for a in 5..limit {
            if sieve[a as usize] {
                primes.push(a)
            }
        }

        primes
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Application::new();
        QApplication::exec()
    })
}
