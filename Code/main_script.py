from review_tool import Ui_MainWindow
from benchmark_win import Ui_message_box
from PyQt5 import QtGui,QtWidgets,QtCore
import sys
import hashlib

class main_window(QtWidgets.QMainWindow,Ui_MainWindow):

    def __init__(self,parent=None):
        super(main_window,self).__init__(parent)
        self.setupUi(self)
        self.commandLinkButton.clicked.connect(self.open_benchmark)
        self.pushButton_3.clicked.connect(self.open_file)
        self.pushButton.clicked.connect(self.get_analysis)
        self.tabWidget = QtWidgets.QTableWidget()
        finish = QtWidgets.QAction("Quit", self)
        finish.triggered.connect(self.close_event)
        self.actionConnect_To_IP.setShortcut('Ctrl+S')
        self.actionConnect_To_IP.triggered.connect(self.saveFile)
        self.actionExit_2.setShortcut('Ctrl+W')
        self.actionExit_2.triggered.connect(self.close_event)


    def open_benchmark(self):
        bench_window = benchmark_window()
        bench_window.exec_()

    def open_file(self):
        options = QtWidgets.QFileDialog.Options()
        options |= QtWidgets.QFileDialog.DontUseNativeDialog
        self.file_name, _  = QtWidgets.QFileDialog.getOpenFileName(self,"Select File to Upload",options=options)
        self.lineEdit.setText(self.file_name)
        with open(self.file_name,"r") as file:
            content = file.read()
            self.textBrowser.setText(content)
            print(help(self.file_name))

    def close_event(self):
        close_response = showmessagebox(1)
        if close_response == 16384:
            self.close()
            sys.exit()
        else:
            pass

    def get_analysis(self):
        import pycodestyle
        fchecker = pycodestyle.Checker(self.file_name, show_source=True)
        file_errors = fchecker.check_all()

    def saveFile(self):
        name = QtWidgets.QFileDialog.getSaveFileName(self, 'Save File')
        with open(name[0], 'w') as file:
            text = self.textBrowser.toPlainText()
            file.write(text)

    

def showmessagebox(x):
    msg = QtWidgets.QMessageBox()
    msg.setWindowTitle("Status Window")
    msg.setIcon(QtWidgets.QMessageBox.Information)

    if x == 1:
        msg.setText('Are you sure you wanna exit?')
        msg.setIcon(QtWidgets.QMessageBox.Question)
        msg.setStandardButtons(QtWidgets.QMessageBox.Yes | QtWidgets.QMessageBox.No)


    return msg.exec_()



class benchmark_window(QtWidgets.QDialog,Ui_message_box):
    def __init__(self,parent=None):
        super(benchmark_window,self).__init__(parent)
        self.setupUi(self)
        self.goback.pressed.connect(self.close_event)


    def close_event(self):
        self.close()



if __name__ == '__main__':
    app = QtWidgets.QApplication(sys.argv)
    obj = main_window()
    obj.show()
    app.exec()