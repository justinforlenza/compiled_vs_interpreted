import sys

import time

from typing import List

from PyQt5.QtWidgets import QApplication, QWidget, QDesktopWidget, QPushButton, QLineEdit, QTextEdit, QLabel
from PyQt5.QtGui import QIntValidator, QFont


def current_milli_time():
    return round(time.time() * 1000)


class Application(QWidget):

    def __init__(self):
        super().__init__()

        self.init_ui()

    
    def center(self):
        qr = self.frameGeometry()
        cp = QDesktopWidget().availableGeometry().center()
        qr.moveCenter(cp)
        self.move(qr.topLeft())


    def init_ui(self):
        self.resize(300, 300)
        self.setFixedSize(self.size())
        self.center()
        self.setWindowTitle('Prime Number Generator')

        self.textbox = QLineEdit(self)
        self.textbox.setValidator(QIntValidator(1, 9999999))
        self.textbox.move(20, 20)
        self.textbox.resize(260, 35)
        self.textbox.textChanged.connect(self.on_text_changed)
        
        self.btn = QPushButton(self)
        self.btn.move(20, 60)
        self.btn.resize(260, 35)
        self.btn.setText('Generate')
        self.btn.setDisabled(True)
        self.btn.clicked.connect(self.on_btn_clicked)

        self.results_label = QLabel(self)
        self.results_label.move(20, 100)
        self.results_label.setText('Results:')

        self.results = QTextEdit(self)
        self.results.setFontFamily('monospace')
        self.results.move(20, 120)
        self.results.resize(260, 160)
        self.results.setReadOnly(True)
        
        self.show()
    
    def on_text_changed(self):
        try:
            self.btn.setDisabled(int(self.textbox.text()) < 1)
        except ValueError:
            self.btn.setDisabled(True)
    
    def on_btn_clicked(self):
        start = current_milli_time()

        primes = self._sieve_of_atkin(int(self.textbox.text()))
        primes_count = len(primes)

        elapsed = current_milli_time() - start

        result_text = [
            f'Time         : {elapsed}ms',
            f'Total Found  : {primes_count}',
            '',
            'Numbers:'
        ]
        
        for elem in primes:
            result_text.append(str(elem))
        
        self.results.setText('\n'.join(result_text))

    @staticmethod
    def _sieve_of_atkin(limit: int) -> List[int]:
        primes = []

        if limit >= 2:
            primes.append(2)
        if limit >= 3:
            primes.append(3)
        if limit == 5:
            primes.append(5)

        if limit <= 5:
            return primes

        sieve = [False] * limit
        for i in range(0, limit):
            sieve[i] = False
        x = 1
        while x * x < limit:
            y = 1
            while y * y < limit:
                n = (4 * x * x) + (y * y)
                if (n <= limit and (n % 12 == 1 or
                                    n % 12 == 5)):
                    sieve[n] ^= True

                n = (3 * x * x) + (y * y)
                if n <= limit and n % 12 == 7:
                    sieve[n] ^= True

                n = (3 * x * x) - (y * y)
                if (x > y and n <= limit and
                        n % 12 == 11):
                    sieve[n] ^= True
                y += 1
            x += 1

        r = 5
        while r * r < limit:
            if sieve[r]:
                for i in range(r * r, limit, r * r):
                    sieve[i] = False
            r += 1

        for a in range(5, limit):
            if sieve[a]:
                primes.append(a)
        
        return primes

def main():

    app = QApplication(sys.argv)
    ex = Application()
    sys.exit(app.exec_())

if __name__ == '__main__':
    main()