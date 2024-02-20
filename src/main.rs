slint::slint! {
    import { VerticalBox, LineEdit, ScrollView, HorizontalBox } from "std-widgets.slint";
    struct Status {
        date-time: string,        
        health-power: int,
        health: string,
    }
    component StatusBar inherits Rectangle {
        background: lightblue;
        in property <Status> status;
        HorizontalBox {
            padding: 2px;
            Text {
                text: status.date-time;
                color: yellow;
                font-size: 9px;
            }
            Text {
                text: status.health-power;
                font-size: 9px;
            }
            Text {
                text: status.health;
                color: status.health-power < 20? red: status.health-power < 50? yellow: white;
                font-size: 9px;
            }
        }
    }
    export  component RoundBorder inherits Rectangle {
        border-color: gray;
        border-width: 1px;
        border-radius: 5px;        
    }
    export component MainConsole inherits Window {
        in property <int> width_: 400;
        in property <int> height_: 300;
        width: width_ * 1px;
        height: max(200, height_) * 1px;
        in-out property <string> content;
        in property <Status> status: {
            date-time: "12:00:01",
            health-power: 100,
            health: "Normal",
         };
        background: white;
        VerticalBox {
            StatusBar {
                status: status;
            }
            RoundBorder {
                width: parent.width - parent.padding * 2;
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
    if let Ok(ui) = MainConsole::new() {
        ui.set_content("Hello!! Welcome to this wolrd!\n".into());
        let _ = ui.run();
    }
}
