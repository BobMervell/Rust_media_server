use anyhow::{anyhow, Result};
use std::fs;
use std::process::Command;

pub fn mount_smb(
    user: &str,
    password: &str,
    ip: &str,
    folder_path: &str,
    mount_point: &str,
) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        fs::create_dir_all(mount_point)?;

        Command::new("sudo")
            .arg("mount")
            .arg("-t")
            .arg("cifs")
            .arg(format!("//{}/{}", ip, folder_path))
            .arg(mount_point)
            .arg("-o")
            .arg(format!("username={},password={}", user, password))
            .status()
            .map(|_| ())?;

        return Ok(());
    }
    return Err(anyhow!("Target os not supported: "));
}

pub fn unmount_smb(mount_point: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        Command::new("sudo")
            .arg("umount")
            .arg("-l")
            .arg(mount_point)
            .status()
            .map(|_| ())?;
        return Ok(());
    }
    return Err(anyhow!("Target os not supported: "));
}
