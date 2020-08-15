# -*- coding: utf-8 -*-


from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_message_box(object):
    def setupUi(self, message_box):
        message_box.setObjectName("message_box")
        message_box.resize(842, 483)
        self.gridLayout = QtWidgets.QGridLayout(message_box)
        self.gridLayout.setObjectName("gridLayout")
        self.tabWidget = QtWidgets.QTabWidget(message_box)
        self.tabWidget.setObjectName("tabWidget")
        self.tab = QtWidgets.QWidget()
        self.tab.setObjectName("tab")
        self.tabWidget.addTab(self.tab, "")
        self.tab_2 = QtWidgets.QWidget()
        self.tab_2.setObjectName("tab_2")
        self.tableView = QtWidgets.QTableView(self.tab_2)
        self.tableView.setGeometry(QtCore.QRect(10, 10, 801, 371))
        self.tableView.setObjectName("tableView")
        self.tabWidget.addTab(self.tab_2, "")
        self.gridLayout.addWidget(self.tabWidget, 1, 0, 1, 1)
        self.goback = QtWidgets.QCommandLinkButton(message_box)
        self.goback.setObjectName("goback")
        self.gridLayout.addWidget(self.goback, 0, 0, 1, 1)

        self.retranslateUi(message_box)
        self.tabWidget.setCurrentIndex(1)
        QtCore.QMetaObject.connectSlotsByName(message_box)

    def retranslateUi(self, message_box):
        _translate = QtCore.QCoreApplication.translate
        message_box.setWindowTitle(_translate("message_box", "Dialog"))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab), _translate("message_box", "Visualization"))
        self.tabWidget.setTabText(self.tabWidget.indexOf(self.tab_2), _translate("message_box", "Benchmark Table"))
        self.goback.setText(_translate("message_box", "Go Back"))

