fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
        component BootloaderScreen inherits Rectangle {
        width: 491px;
        height: 307px;

        background: black;
        Text {
            text: "Created by KimWang906.";
            color: white;
            accessible-value-maximum: 3000;
        }
    }
    export component MainWindow inherits Window {
        width: 406px;
        height: 509px;
        BootloaderScreen {}
    }
}
