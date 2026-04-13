// Подключаем необходимые типы из библиотек
use eframe::egui;
use egui::pos2;

// 1. Структура, которая будет хранить состояние нашего приложения
struct LongLine
{
    color: egui::Color32,
    line: Vec<egui::Pos2>,
}



struct MyApp {
    line: Vec<LongLine>,
    temp_line: Option<Vec<egui::Pos2>>,
    temp_point: Option<egui::Pos2>,
    flag_press: bool,

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            line: Vec::new(),
            temp_line: None,
            temp_point: None,
            flag_press: false,
        }
    }
}

fn smooth_line(_start: egui::Pos2,_end: egui::Pos2) -> egui::Pos2
{

    if (_start.x - _end.x).abs() > (_start.y - _end.y).abs() {
        return pos2(_end.x, _start.y);
    } else {
        return pos2(_start.x, _end.y);
    }

}

// 2. Реализуем трейт eframe::App для нашей структуры
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        ctx.input(|i| {
            // Обработка нажатий кнопки
            for event in &i.events {
                if let egui::Event::PointerButton { pos, button: _button, pressed, modifiers: _modifiers } = event {
                    
                    if *pressed && self.temp_line.is_none() && (*_button == egui::PointerButton::Primary)
                    {
                        let mut new_vec = Vec::new();
                        new_vec.push(*pos);
                        self.temp_line = Some(new_vec);
                        self.flag_press = true;
                        
                        
                    }
                    
                    if *pressed && !self.temp_line.is_none() && (*_button == egui::PointerButton::Primary)
                    {
                        if let Some(vec_line) = &mut self.temp_line
                        {
                            if let Some(last_point) = vec_line.last(){
                                vec_line.push(smooth_line(*last_point,*pos));
                            }
                            
                        }
                    }

                    if *pressed && (*_button == egui::PointerButton::Secondary)
                    {
                        if let Some(coly_line) = self.temp_line.take()  {

                                let long_line = LongLine{color: egui::Color32::GREEN,line:coly_line };
                                self.line.push(long_line);
                        }
                        self.flag_press=false;
                        self.temp_line=None;
                        self.temp_point=None;
                        


                    }
                                      
                   
                }
            }
            
            // Обработка движения мыши
            for event in &i.events {
                if let egui::Event::PointerMoved(pos) = event {
                    if self.flag_press {

                        if let Some(temp_line_vec) = &self.temp_line
                        {
                            if let Some(start) = temp_line_vec.last()
                            {
                                self.temp_point = Some(smooth_line(*start,*pos));
                            }
                            
                        }
                        
                        
                    
                        
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            if self.flag_press{
                if let  Some(vec_line) = &self.temp_line  {
                    if vec_line.len() >= 2{
                        for line in vec_line.windows(2)  {
                            painter.add(
                                egui::Shape::LineSegment {
                                    points: [line[0], line[1]],
                                    stroke: egui::Stroke::new(3.0, egui::Color32::BLACK), // Синий для сохраненных линий
                                }
                            );
                        }
                    }

                    if let (Some(start), Some(end)) = (vec_line.last(), self.temp_point) {
                        painter.add(
                            egui::Shape::LineSegment {
                                points: [*start, end],
                                stroke: egui::Stroke::new(3.0, egui::Color32::BLACK), // Красный для предварительной линии
                            }
                        );
                    }
                }

            }

            if !self.line.is_empty() {
                for vec_line in &self.line
                {
                    for lines in vec_line.line.windows(2)
                    {
                        painter.add(
                            egui::Shape::LineSegment {
                                points: [lines[0], lines[1]],
                                stroke: egui::Stroke::new(3.0, vec_line.color), // Красный для предварительной линии
                            }
                        );
                    }

                }
            }

        });        
    }
}

// 3. Точка входа в программу
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Logsim",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}