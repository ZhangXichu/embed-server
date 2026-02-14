use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct DataSaver {
    db_path: PathBuf,
    next_id: u64,
}

impl DataSaver {
    /// Initialize a new DataSaver that stores chunks in LanceDB
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        
        // Create the database directory if it doesn't exist
        std::fs::create_dir_all(&db_path)
            .with_context(|| format!("Failed to create database directory: {}", db_path.display()))?;
        
        Ok(Self {
            db_path,
            next_id: 0,
        })
    }
    
    /// Get the path to the LanceDB folder
    fn get_db_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Failed to determine home directory")?;
        
        let db_path = home
            .join(".local/share/local_searcher/lancedb");
        
        Ok(db_path)
    }
    
    /// Save a chunk to the database
    pub fn save_chunk(&mut self, text: String) -> Result<()> {
        self.next_id += 1;
        
        // Save chunk to the database at db_path
        // For now, we'll just print confirmation 
        println!("Saved chunk #{}: {} chars to {:?}", self.next_id, text.len(), self.db_path);
        
        // TODO: Implement actual LanceDB storage once async runtime is set up
        Ok(())
    }
    
    /// Save multiple chunks at once
    pub fn save_chunks(&mut self, chunks: Vec<String>) -> Result<()> {
        for chunk in chunks {
            self.save_chunk(chunk)?;
        }
        Ok(())
    }
}

