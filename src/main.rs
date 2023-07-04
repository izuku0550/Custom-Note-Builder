use std::{path::Path, sync::Arc};

use slint::{Image, SharedString};
use windows::{
    core::{HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, MAX_PATH},
        UI::{
            Controls::Dialogs::{
                GetOpenFileNameW, OPENFILENAMEW, OPEN_FILENAME_FLAGS, OPEN_FILENAME_FLAGS_EX,
            },
            Shell::ShellExecuteW,
            WindowsAndMessaging::SW_SHOWMAXIMIZED,
        },
    },
};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let weak = Arc::new(main_window.as_weak());

    // Load Icon Path
    let wrap_weak = Arc::clone(&weak);
    main_window
        .global::<CustomNoteBuilder_Logic>()
        .on_change_icon_pressed(move || {
            let app = wrap_weak.unwrap();
            let data = &mut [0u16; MAX_PATH as usize];
            let mut ofn = OPENFILENAMEW {
                lStructSize: std::mem::size_of::<OPENFILENAMEW>()
                    .try_into()
                    .expect("Failed convert OPENFILENAMEW"),
                hwndOwner: HWND::default(),
                hInstance: HMODULE(0),
                lpstrFilter: PCWSTR(
                    HSTRING::from(
                        "Icon Files (*.ico)\0*.ico\0All Files (*.*)\0*.*\0".trim_end_matches('\0'),
                    )
                    .as_ptr(),
                ),
                lpstrCustomFilter: PWSTR::null(),
                nMaxCustFilter: 0,
                nFilterIndex: 0,
                lpstrFile: PWSTR(data.as_mut_ptr()),
                nMaxFile: MAX_PATH,
                lpstrFileTitle: PWSTR::null(),
                nMaxFileTitle: 0,
                lpstrInitialDir: PCWSTR::null(),
                lpstrTitle: PCWSTR::null(),
                Flags: OPEN_FILENAME_FLAGS(0),
                nFileOffset: 0,
                nFileExtension: 0,
                lpstrDefExt: PCWSTR::null(),
                lCustData: LPARAM(0),
                lpfnHook: None,
                lpTemplateName: PCWSTR::null(),
                pvReserved: std::ptr::null_mut(),
                dwReserved: 0,
                FlagsEx: OPEN_FILENAME_FLAGS_EX(0),
            };

            // Get path & filename
            if unsafe { GetOpenFileNameW(&mut ofn).as_bool() } {
                let data = String::from_utf16_lossy(data);
                let convert_data = data.trim_end_matches('\0');
                let path = Path::new(&convert_data);
                let file_name = path.file_name().unwrap().to_str().unwrap();
                app.global::<CustomNoteBuilder_Logic>()
                    .set_file_name(SharedString::from(file_name.replace("\0", "")));
                app.global::<CustomNoteBuilder_Logic>()
                    .set_path(Image::load_from_path(path).expect("Failed load to image"));
                app.window().request_redraw();
            }
        });

    // Delete filename in input box
    let wrap_weak = Arc::clone(&weak);
    main_window
        .global::<CustomNoteBuilder_Logic>()
        .on_path_clear_pressed(move || {
            let app = wrap_weak.unwrap();
            app.global::<CustomNoteBuilder_Logic>()
                .set_file_name(SharedString::default());
            app.global::<CustomNoteBuilder_Logic>()
                .set_path(Image::default());
            app.window().request_redraw();
        });

    // Get Information
    let wrap_weak = Arc::clone(&weak);
    main_window
        .global::<CustomNoteBuilder_Logic>()
        .on_info_pressed(move || {
            let _app = wrap_weak.unwrap();
            unsafe {
                ShellExecuteW(
                    HWND::default(),
                    PCWSTR(HSTRING::from("open".trim_end_matches('\0')).as_ptr()),
                    PCWSTR(
                        HSTRING::from(
                            "https://github.com/izuku0550/Custom-Note-Builder"
                                .trim_end_matches('\0'),
                        )
                        .as_ptr(),
                    ),
                    PCWSTR::null(),
                    PCWSTR::null(),
                    SW_SHOWMAXIMIZED,
                );
            }
        });
    main_window.run()
}
