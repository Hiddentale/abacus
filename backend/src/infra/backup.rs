use sqlx::SqlitePool;
use std::error::Error;
use std::path::{Path, PathBuf};

pub struct BackupConfig {
    pub directory: PathBuf,
}

async fn backup_database(pool: &SqlitePool, destination: &Path) -> Result<(), Box<dyn Error>> {
    let destination = destination
        .to_str()
        .ok_or("error formatting string")?
        .to_string();
    let mut connection = pool.acquire().await?;
    sqlx::query("VACUUM INTO ?")
        .bind(destination)
        .execute(&mut *connection)
        .await?;
    Ok(())
}

fn create_backup_directory(backupconfig: &BackupConfig) -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all(&backupconfig.directory)?;
    Ok(())
}

pub fn store_backup_directory_path(backupconfig: &mut BackupConfig) -> Result<(), Box<dyn Error>> {
    let executable_path = std::env::current_exe()?;
    let executable_directory = executable_path.parent().ok_or("parent does not exist")?;
    let backup_directory = executable_directory.join("database_backups");
    backupconfig.directory = backup_directory;
    Ok(())
}

pub async fn create_local_backup(
    pool: &SqlitePool,
    backupconfig: &BackupConfig,
) -> Result<(), Box<dyn Error>> {
    backup_database(pool, &backupconfig.directory).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_backup_config() -> BackupConfig {
        let parent_directory = std::env::temp_dir();
        let backup_directory = parent_directory.join("database_backups");
        let mut backup_config = BackupConfig {
            directory: backup_directory,
        };
        store_backup_directory_path(&mut backup_config).unwrap();
        backup_config
    }

    fn delete_backup_directory() {
        !todo()
    }

    #[test]
    fn backup_directory_is_created_succesfully() {
        let backup_config = setup_backup_config();
        create_backup_directory(&backup_config).unwrap();
        //delete_backup_directory();
    }
    #[sqlx::test]
    async fn local_backup_is_succesfull(pool: SqlitePool) {
        let backup_config = setup_backup_config();
        create_backup_directory(&backup_config).unwrap();
        create_local_backup(&pool, &backup_config).await.unwrap();
        //delete_backup_directory();
    }
}
