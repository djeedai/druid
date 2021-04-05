// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;

use druid_shell::kurbo::{Line, Point, Size};
use druid_shell::piet::{Color, RenderContext};

use druid_shell::{
    Application, Cursor, FileDialogOptions, FileDialogToken, FileInfo, FileSpec, HotKey, KeyEvent,
    Menu, MouseEvent, Region, SysMods, TimerToken, WinHandler, WindowBuilder, WindowHandle,
};

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0x80, 0x8a);
const BG_COLOR_CHILD: Color = Color::rgb8(0x47, 0x48, 0x42);

const MENU_EXIT: u32 = 0x100;
const MENU_OPEN: u32 = 0x101;
const MENU_LAYOUT: u32 = 0x102;

#[derive(Default)]
struct HelloState {
    size: Size,
    handle: WindowHandle,
}

impl WinHandler for HelloState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut piet_common::Piet, _: &Region) {
        let rect = self.size.to_rect();
        piet.fill(rect, &BG_COLOR);
        piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
    }

    fn command(&mut self, id: u32) {
        match id {
            MENU_EXIT => {
                self.handle.close();
                Application::global().quit()
            }
            MENU_OPEN => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                self.handle.open_file(options);
            }
            MENU_LAYOUT => {}
            _ => println!("unexpected id {}", id),
        }
    }

    fn open_file(&mut self, _token: FileDialogToken, file_info: Option<FileInfo>) {
        println!("open file result: {:?}", file_info);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        println!("mouse_wheel {:?}", event);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn got_focus(&mut self) {
        println!("Got focus");
    }

    fn lost_focus(&mut self) {
        println!("Lost focus");
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct ChildHandler {
    size: Size,
    handle: WindowHandle,
    parent_handle: WindowHandle,
}

impl WinHandler for ChildHandler {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut piet_common::Piet, _: &Region) {
        println!("child paint");
        let rect = self.size.to_rect();
        piet.fill(rect, &BG_COLOR_CHILD);
        piet.stroke(Line::new((10.0, 90.0), (90.0, 50.0)), &FG_COLOR, 1.0);
    }

    fn paint_raw(&mut self) {
        println!("child paint_raw");
    }

    fn command(&mut self, id: u32) {
        match id {
            _ => println!("child unexpected id {}", id),
        }
    }

    fn open_file(&mut self, _token: FileDialogToken, file_info: Option<FileInfo>) {
        println!("child open file result: {:?}", file_info);
    }

    fn key_down(&mut self, event: KeyEvent) -> bool {
        println!("child keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: KeyEvent) {
        println!("child keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &MouseEvent) {
        println!("child mouse_wheel {:?}", event);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        println!("child mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("child mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("child mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("child timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        println!("child size {:?}", size);
        self.size = size;
    }

    fn got_focus(&mut self) {
        println!("child Got focus");
    }

    fn lost_focus(&mut self) {
        println!("child Lost focus");
    }

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        println!("child destroy");
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    let mut file_menu = Menu::new();
    file_menu.add_item(
        MENU_OPEN,
        "O&pen",
        Some(&HotKey::new(SysMods::Cmd, "o")),
        true,
        false,
    );
    file_menu.add_item(
        MENU_EXIT,
        "E&xit",
        Some(&HotKey::new(SysMods::Cmd, "q")),
        true,
        false,
    );
    file_menu.add_item(
        MENU_LAYOUT,
        "Alt &Layout",
        Some(&HotKey::new(SysMods::Cmd, "l")),
        true,
        false,
    );
    let mut menubar = Menu::new();
    menubar.add_dropdown(file_menu, "&File", true);

    let app = Application::new().unwrap();

    let mut builder = WindowBuilder::new(app.clone());
    builder.set_handler(Box::new(HelloState::default()));
    builder.set_title("Child window example");
    builder.set_menu(menubar);
    let window = builder.build().unwrap();

    let child_handler = ChildHandler {
        size: Size::default(),
        handle: WindowHandle::default(),
        parent_handle: window.clone(),
    };
    let mut child_builder = WindowBuilder::new(app.clone());
    child_builder.set_handler(Box::new(child_handler));
    child_builder.set_parent(&window);
    child_builder.set_position(Point::new(10., 10.));
    child_builder.set_size(Size::new(200., 150.));
    //child_builder.set_has_render_target(false);
    let child_window = child_builder.build().unwrap();
    child_window.show();

    window.show();

    app.run(None);
}
