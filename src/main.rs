slint::slint! {
    import { VerticalBox, LineEdit, ScrollView } from "std-widgets.slint";
    export  component RoundBorder inherits Rectangle {
        border-color: gray;
        border-width: 1px;
        border-radius: 5px;        
    }
export component MainConsole inherits Window {
        in property <int> width_;
        in property <int> height_;
        width: width_ * 1px;
        height: max(200, height_) * 1px;
        in-out property <string> content;
        background: white;
        VerticalBox {
            RoundBorder {
                width: parent.width - parent.padding * 2;
                height: parent.height - 50px;
                clip: true;
                ScrollView {
                    width: parent.width;
                    height: parent.height;
                    VerticalBox {
                        //alignment: top;
                        Text {
                            text: content;
                            color: black;
                            wrap: word-wrap;
                            font-size: 16px;
                        }
                    }
                }
            }
            spacing: 5px;
            RoundBorder {
                width: root.width - parent.padding * 2;
                height: 30px;
                line_edit := LineEdit {
                    width: parent.width;
                    height: parent.height;
                    placeholder-text: "Input here..";
                    accepted(s) => {
                        root.content += s + "\n";
                        self.text = "";
                    }
                    
                }
            }            
        }    
        forward-focus: line_edit;    
    }
}

fn main() {
    println!("Hello, world!");
    if let Ok(ui) = MainConsole::new() {
        ui.set_width_(400);
        ui.set_height_(300);
        ui.set_content("Hello!! Welcome to this wolrd!\n".into());
        let _ = ui.run();
    }
}
