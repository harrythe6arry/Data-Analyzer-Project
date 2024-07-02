use crate::errors::*;
use crate::traits::{Close, High, Low, Open, Volume};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Data item is used as an input for indicators.
///
/// # Example
///
/// ```
/// use ta::DataItem;
/// use ta::{Open, High, Low, Close, Volume};
///
/// let item = DataItem::builder()
///     .open(20.0)
///     .high(25.0)
///     .low(15.0)
///     .close(21.0)
///     .volume(7500.0)
///     .build()
///     .unwrap();
///
/// assert_eq!(item.open(), 20.0);
/// assert_eq!(item.high(), 25.0);
/// assert_eq!(item.low(), 15.0);
/// assert_eq!(item.close(), 21.0);
/// assert_eq!(item.volume(), 7500.0);
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct DataItem {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl DataItem {
    pub fn builder() -> DataItemBuilder {
        DataItemBuilder::new()
    }
}

impl Open for DataItem {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for DataItem {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for DataItem {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for DataItem {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for DataItem {
    fn volume(&self) -> f64 {
        self.volume
    }
}

pub struct DataItemBuilder {
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    volume: Option<f64>,
}

impl DataItemBuilder {
    pub fn new() -> Self {
        Self {
            open: None,
            high: None,
            low: None,
            close: None,
            volume: None,
        }
    }

    pub fn open(mut self, val: f64) -> Self {
        self.open = Some(val);
        self
    }

    pub fn high(mut self, val: f64) -> Self {
        self.high = Some(val);
        self
    }

    pub fn low(mut self, val: f64) -> Self {
        self.low = Some(val);
        self
    }

    pub fn close(mut self, val: f64) -> Self {
        self.close = Some(val);
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.volume = Some(val);
        self
    }

    pub fn build(self) -> Result<DataItem> {
        if let (Some(open), Some(high), Some(low), Some(close), Some(volume)) =
            (self.open, self.high, self.low, self.close, self.volume)
        {
            // validate
            if low <= open
                && low <= close
                && low <= high
                && high >= open
                && high >= close
                && volume >= 0.0
            {
                let item = DataItem {
                    open,
                    high,
                    low,
                    close,
                    volume,
                };
                Ok(item)
            } else {
                Err(TaError::DataItemInvalid)
            }
        } else {
            Err(TaError::DataItemIncomplete)
        }
    }
}