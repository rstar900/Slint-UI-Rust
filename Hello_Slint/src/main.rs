fn main() {
    HelloSlint::new().unwrap().run().unwrap();
}

// description of the visual component in slint's language using macro
slint::slint! {
    export component HelloSlint inherits Window {
        title: "Hello Slint";
        background: blue;
        width: 640px;
        height: 480px;
        Text {
            text : "Hello, Slint!";
            color: yellow;
            font-size: 50px;
            font-weight: 50;
            vertical-alignment: center;
            horizontal-alignment: center;

        }
    }
}