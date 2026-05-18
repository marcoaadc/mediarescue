use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectorStatus {
    Unread,
    Good,
    Bad,
    SignatureFound,
}

pub struct SectorMap {
    statuses: Vec<SectorStatus>,
}

impl SectorMap {
    pub fn new(total_sectors: u64) -> Self {
        Self {
            statuses: vec![SectorStatus::Unread; total_sectors as usize],
        }
    }

    pub fn set(&mut self, sector: u64, status: SectorStatus) {
        if (sector as usize) < self.statuses.len() {
            self.statuses[sector as usize] = status;
        }
    }

    pub fn get(&self, sector: u64) -> SectorStatus {
        self.statuses
            .get(sector as usize)
            .copied()
            .unwrap_or(SectorStatus::Unread)
    }

    pub fn total(&self) -> u64 {
        self.statuses.len() as u64
    }

    pub fn count_by_status(&self, status: SectorStatus) -> u64 {
        self.statuses.iter().filter(|s| **s == status).count() as u64
    }

    pub fn scanned_count(&self) -> u64 {
        self.statuses
            .iter()
            .filter(|s| **s != SectorStatus::Unread)
            .count() as u64
    }

    pub fn progress_percent(&self) -> f32 {
        if self.statuses.is_empty() {
            return 0.0;
        }
        (self.scanned_count() as f32 / self.statuses.len() as f32) * 100.0
    }

    pub fn to_summary_grid(&self, width: u32) -> Vec<SectorStatus> {
        if self.statuses.is_empty() || width == 0 {
            return Vec::new();
        }

        let sectors_per_cell = (self.statuses.len() as f64 / width as f64).ceil() as usize;
        let mut grid = Vec::with_capacity(width as usize);

        for chunk in self.statuses.chunks(sectors_per_cell.max(1)) {
            let status = if chunk.iter().any(|s| *s == SectorStatus::Bad) {
                SectorStatus::Bad
            } else if chunk.iter().any(|s| *s == SectorStatus::SignatureFound) {
                SectorStatus::SignatureFound
            } else if chunk.iter().all(|s| *s == SectorStatus::Good) {
                SectorStatus::Good
            } else {
                SectorStatus::Unread
            };
            grid.push(status);
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sector_map_all_unread() {
        let map = SectorMap::new(100);
        assert_eq!(map.total(), 100);
        assert_eq!(map.scanned_count(), 0);
        assert_eq!(map.get(0), SectorStatus::Unread);
    }

    #[test]
    fn test_set_and_get() {
        let mut map = SectorMap::new(10);
        map.set(5, SectorStatus::Good);
        assert_eq!(map.get(5), SectorStatus::Good);
        assert_eq!(map.get(4), SectorStatus::Unread);
    }

    #[test]
    fn test_progress() {
        let mut map = SectorMap::new(10);
        map.set(0, SectorStatus::Good);
        map.set(1, SectorStatus::Good);
        map.set(2, SectorStatus::Bad);
        assert!((map.progress_percent() - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_count_by_status() {
        let mut map = SectorMap::new(10);
        map.set(0, SectorStatus::Good);
        map.set(1, SectorStatus::Good);
        map.set(2, SectorStatus::Bad);
        map.set(3, SectorStatus::SignatureFound);
        assert_eq!(map.count_by_status(SectorStatus::Good), 2);
        assert_eq!(map.count_by_status(SectorStatus::Bad), 1);
        assert_eq!(map.count_by_status(SectorStatus::SignatureFound), 1);
    }

    #[test]
    fn test_summary_grid() {
        let mut map = SectorMap::new(10);
        for i in 0..5 {
            map.set(i, SectorStatus::Good);
        }
        map.set(7, SectorStatus::Bad);

        let grid = map.to_summary_grid(5);
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0], SectorStatus::Good);
    }
}
