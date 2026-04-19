use eframe::egui;
use egui::{Rect, Scene, Ui, Color32, Stroke, Pos2, Vec2};

struct MyApp {
    // Состояние камеры Scene
    scene_rect: Rect,
    
    // Данные для рисования - только точки
    points: Vec<Pos2>,
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
            points: vec![
                Pos2::new(0.0, 0.0),
                Pos2::new(100.0, 100.0),
                Pos2::new(200.0, 50.0),
                Pos2::new(-100.0, 150.0),
                Pos2::new(-50.0, -50.0),
                Pos2::new(150.0, -100.0),
                Pos2::new(-150.0, -100.0),
                Pos2::new(50.0, 200.0),
            ],
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
            ui.label(format!("Количество точек: {}", self.points.len()));
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
                self.points.clear();
            }
            
            if ui.button("➕ Добавить точку (100, 100)").clicked() {
                self.points.push(Pos2::new(100.0, 100.0));
            }
            
            if ui.button("➕ Добавить точку (-100, -100)").clicked() {
                self.points.push(Pos2::new(-100.0, -100.0));
            }
            
            if ui.button("➕ Добавить точку (200, -150)").clicked() {
                self.points.push(Pos2::new(200.0, -150.0));
            }
        });
        
        // Центральная панель со Scene - используем show_inside
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let current_rect = self.scene_rect;
            
            Scene::new()
                .zoom_range(0.1..=20.0)
                .show(ui, &mut self.scene_rect, |scene_ui| {
                    let painter = scene_ui.painter();
                    
                    // Рисуем сетку
                    draw_grid(&painter, current_rect);
                    
                    // Рисуем координатные оси
                    draw_axes(&painter);
                    
                    // Рисуем только точки
                    for (i, point) in self.points.iter().enumerate() {
                        // Рисуем точку с разным цветом
                        let color = get_point_color(i);
                        let size = 8.0;
                        
                        // Залитая точка
                        painter.circle_filled(*point, size, color);
                        
                        // Обводка точки
                        painter.circle_stroke(*point, size, Stroke::new(2.0, Color32::WHITE));
                        
                        // Подпись точки
                        painter.text(
                            *point + Vec2::new(12.0, -12.0),
                            egui::Align2::LEFT_CENTER,
                            format!("{}", i + 1),
                            egui::FontId::proportional(12.0),
                            Color32::WHITE,
                        );
                    }
                    
                    // Отображение координат под курсором - исправленный вызов input
                    let input = scene_ui.ctx().input(|i| i.clone());
                    if let Some(cursor_pos) = input.pointer.hover_pos() {
                        painter.text(
                            cursor_pos + Vec2::new(15.0, -15.0),
                            egui::Align2::LEFT_CENTER,
                            format!("({:.0}, {:.0})", cursor_pos.x, cursor_pos.y),
                            egui::FontId::proportional(12.0),
                            Color32::from_rgb(200, 200, 100),
                        );
                    }
                });
        });
    }
}

// Функция для получения цвета точки в зависимости от индекса
fn get_point_color(index: usize) -> Color32 {
    match index % 6 {
        0 => Color32::RED,
        1 => Color32::GREEN,
        2 => Color32::BLUE,
        3 => Color32::YELLOW,
        4 => Color32::ORANGE,
        5 => Color32::from_rgb(255, 0, 255), // Magenta
        _ => Color32::WHITE,
    }
}

// Функция для рисования сетки
fn draw_grid(painter: &egui::Painter, visible_rect: Rect) {
    let step = 50.0;
    
    let x_start = (visible_rect.min.x / step).floor() as i32 * step as i32;
    let x_end = (visible_rect.max.x / step).ceil() as i32 * step as i32;
    let y_start = (visible_rect.min.y / step).floor() as i32 * step as i32;
    let y_end = (visible_rect.max.y / step).ceil() as i32 * step as i32;
    
    // Вертикальные линии
    for x in (x_start..=x_end).step_by(step as usize) {
        let x = x as f32;
        let color = if x.abs() < 0.1 {
            Color32::from_rgb(200, 100, 100)
        } else {
            Color32::from_gray(60)
        };
        
        painter.line_segment(
            [Pos2::new(x, visible_rect.min.y), Pos2::new(x, visible_rect.max.y)],
            Stroke::new(1.0, color),
        );
    }
    
    // Горизонтальные линии
    for y in (y_start..=y_end).step_by(step as usize) {
        let y = y as f32;
        let color = if y.abs() < 0.1 {
            Color32::from_rgb(100, 200, 100)
        } else {
            Color32::from_gray(60)
        };
        
        painter.line_segment(
            [Pos2::new(visible_rect.min.x, y), Pos2::new(visible_rect.max.x, y)],
            Stroke::new(1.0, color),
        );
    }
}

// Функция для рисования осей координат
fn draw_axes(painter: &egui::Painter) {
    // Ось X
    painter.line_segment(
        [Pos2::new(-1000.0, 0.0), Pos2::new(1000.0, 0.0)],
        Stroke::new(2.5, Color32::from_rgb(255, 100, 100)),
    );
    
    // Ось Y
    painter.line_segment(
        [Pos2::new(0.0, -1000.0), Pos2::new(0.0, 1000.0)],
        Stroke::new(2.5, Color32::from_rgb(100, 255, 100)),
    );
    
    // Стрелка на оси X
    painter.line_segment(
        [Pos2::new(1000.0, 0.0), Pos2::new(990.0, -6.0)],
        Stroke::new(2.5, Color32::from_rgb(255, 100, 100)),
    );
    painter.line_segment(
        [Pos2::new(1000.0, 0.0), Pos2::new(990.0, 6.0)],
        Stroke::new(2.5, Color32::from_rgb(255, 100, 100)),
    );
    
    // Стрелка на оси Y
    painter.line_segment(
        [Pos2::new(0.0, 1000.0), Pos2::new(-6.0, 990.0)],
        Stroke::new(2.5, Color32::from_rgb(100, 255, 100)),
    );
    painter.line_segment(
        [Pos2::new(0.0, 1000.0), Pos2::new(6.0, 990.0)],
        Stroke::new(2.5, Color32::from_rgb(100, 255, 100)),
    );
    
    // Подписи осей
    painter.text(
        Pos2::new(1020.0, -15.0),
        egui::Align2::LEFT_TOP,
        "X",
        egui::FontId::proportional(18.0),
        Color32::from_rgb(255, 150, 150),
    );
    
    painter.text(
        Pos2::new(15.0, 1020.0),
        egui::Align2::LEFT_TOP,
        "Y",
        egui::FontId::proportional(18.0),
        Color32::from_rgb(150, 255, 150),
    );
    
    // Подпись центра
    painter.text(
        Pos2::new(5.0, -15.0),
        egui::Align2::LEFT_TOP,
        "0",
        egui::FontId::proportional(14.0),
        Color32::from_gray(180),
    );
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