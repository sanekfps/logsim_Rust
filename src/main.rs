use eframe::egui;
use egui::{Color32, Pos2, Rect, Scene, Ui, Vec2};


struct LongLine
{
    color: egui::Color32,
    line: Vec<egui::Pos2>,
}

struct MyApp {
    // Состояние камеры Scene
    scene_rect: Rect,
    
    line: Vec<LongLine>,
    temp_line: Option<Vec<egui::Pos2>>,
    temp_point: Option<egui::Pos2>,
    flag_press: bool,
    

}

fn smooth_line(_start: egui::Pos2,_end: egui::Pos2) -> egui::Pos2
{

    if (_start.x - _end.x).abs() > (_start.y - _end.y).abs() {
        return egui::pos2(_end.x, _start.y);
    } else {
        return egui::pos2(_start.x, _end.y);
    }

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Начальная видимая область: от (-500, -500) до (500, 500)
            scene_rect: Rect::from_min_size(
                Pos2::new(-500.0, -500.0),
                Vec2::new(1000.0, 1000.0)
            ),
            // Только точки без соединений


            line: Vec::new(),
            temp_line: None,
            temp_point: None,
            flag_press: false,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Левая панель с информацией - используем show_inside

        
        egui::Panel::left("left_panel").show_inside(ui, |ui| {
            ui.heading("Управление");
            ui.label("🖱️ ПКМ + Drag: Панорамирование");
            ui.label("🖱️ Колесико мыши: Масштабирование");
            ui.separator();
            
            ui.heading("Информация");
            ui.separator();
            
            ui.heading("Видимая область:");
            ui.label(format!("  X: {:.0}..{:.0}", 
                self.scene_rect.min.x, self.scene_rect.max.x));
            ui.label(format!("  Y: {:.0}..{:.0}", 
                self.scene_rect.min.y, self.scene_rect.max.y));
            ui.separator();
            
            if ui.button("🔄 Сбросить вид").clicked() {
                self.scene_rect = Rect::from_min_size(
                    Pos2::new(-500.0, -500.0),
                    Vec2::new(1000.0, 1000.0)
                );
            }
            
            if ui.button("➕ Приблизить").clicked() {
                let center = self.scene_rect.center();
                let new_size = self.scene_rect.size() * 0.8;
                self.scene_rect = Rect::from_center_size(center, new_size);
            }
            
            if ui.button("➖ Отдалить").clicked() {
                let center = self.scene_rect.center();
                let new_size = self.scene_rect.size() * 1.2;
                self.scene_rect = Rect::from_center_size(center, new_size);
            }
            
            ui.separator();
            if ui.button("🗑️ Очистить все точки").clicked() {
            }
            
            if ui.button("➕ Добавить точку (100, 100)").clicked() {
            }
            
            if ui.button("➕ Добавить точку (-100, -100)").clicked() {
            }
            
            if ui.button("➕ Добавить точку (200, -150)").clicked() {
            }
        });
        
        // Центральная панель со Scene - используем show_inside
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let current_rect = self.scene_rect;
            
            Scene::new()
                .zoom_range(0.1..=20.0)
                .show(ui, &mut self.scene_rect, |scene_ui| {
                    
                    let painter = scene_ui.painter();
                    
                    
                    let layer_transform = scene_ui.ctx().layer_transform_from_global(painter.layer_id())
            .unwrap_or_default();
                    

                    scene_ui.input(|i| {
                        for event in &i.events {
                            if let egui::Event::PointerButton { pos: _, button: _button, pressed, modifiers: _modifiers } = event {
                                if let Some(pointer_global) = i.pointer.latest_pos() {
                                    let local_point = layer_transform * pointer_global;
                                    if *pressed && self.temp_line.is_none() && (*_button == egui::PointerButton::Primary)
                                    {
                                        let mut new_vec = Vec::new();
                                        new_vec.push(local_point);
                                       
                                            

                                        self.temp_line = Some(new_vec);
                                        self.flag_press = true;
                                        
                                        
                                    }
                                    
                                    if *pressed && !self.temp_line.is_none() && (*_button == egui::PointerButton::Primary)
                                    {
                                        if let Some(vec_line) = &mut self.temp_line
                                        {
                                            if let Some(last_point) = vec_line.last(){
                                                vec_line.push(smooth_line(*last_point,local_point));
                                    

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
                        }
                        
                        // Обработка движения мыши
                        for event in &i.events {
                            if let egui::Event::PointerMoved(_pos) = event {
                                if let Some(pointer_global) = i.pointer.latest_pos() {
                                let local_point = layer_transform * pointer_global;

                                    if self.flag_press {

                                        if let Some(temp_line_vec) = &self.temp_line
                                        {
                                            if let Some(start) = temp_line_vec.last()
                                            {
                                                self.temp_point = Some(smooth_line(*start,local_point));
                                            }
                                            
                                        }
                                        
                                        
                                    
                                        
                                    }
                                }
                            }
                        }


                    });


                    
                    // Рисуем сетку
                    
                    draw_grid(&painter, current_rect);

                    painter.add(
                        egui::Shape::LineSegment {
                            points: [egui::pos2(-1000.0, 0.0), egui::pos2(1000.0, 0.0)],
                            stroke: egui::Stroke::new(3.0, egui::Color32::BLUE), // Синий для сохраненных линий
                        }
                    );
                    
                    painter.add(
                        egui::Shape::LineSegment {
                            points: [egui::pos2(0.0,-1000.0), egui::pos2(0.0,1000.0)],
                            stroke: egui::Stroke::new(3.0, egui::Color32::BLUE), // Синий для сохраненных линий
                        }
                    );

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
        });
    }
}


// Функция для рисования сетки из точек
fn draw_grid(painter: &egui::Painter, visible_rect: Rect) {
    let step = 50.0;  // Шаг сетки
    
    // Определяем границы рисования
    let x_start = (visible_rect.min.x / step).floor() as i32 * step as i32;
    let x_end = (visible_rect.max.x / step).ceil() as i32 * step as i32;
    let y_start = (visible_rect.min.y / step).floor() as i32 * step as i32;
    let y_end = (visible_rect.max.y / step).ceil() as i32 * step as i32;
    
    // Рисуем точки сетки
    for x in (x_start..=x_end).step_by(step as usize) {
        for y in (y_start..=y_end).step_by(step as usize) {
            let x = x as f32;
            let y = y as f32;
                        
            // Рисуем точку
            painter.circle_filled(Pos2::new(x, y), 2.0, Color32::from_gray(80));
        }
    }
}


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Точки на координатной плоскости",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}