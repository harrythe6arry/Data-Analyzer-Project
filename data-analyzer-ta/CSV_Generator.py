import csv
import random


def generate_mean_reverting_stock_prices(start_price, total_entries, volatility, mean_reversion_level):
    current_price = start_price
    for i in range(total_entries):
        # Calculate the change with mean reversion
        daily_change = random.uniform(-volatility, volatility)
        current_price += (mean_reversion_level - current_price) * 0.01 + daily_change
        
        open_price = current_price
        close_price = open_price * (1 + daily_change)
        high_price = max(open_price, close_price) * (1 + random.uniform(0, volatility))
        low_price = min(open_price, close_price) * (1 - random.uniform(0, volatility))
        volume = random.uniform(1000, 5000)
        
        yield (i, open_price, high_price, low_price, close_price, volume)

# Generate a large datase
num_entries = 50_000_000 # Total number of entries
start_price = 100.0  # Starting price for the stock
volatility = 0.02  # Volatility factor to ensure consistent fluctuation
mean_reversion_level = 100.0  # Mean reversion level

# Create CSV file
with open('./examples/data/superlarge_dataset.csv', mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(['number', 'open', 'high', 'low', 'close', 'volume'])  # Write header

    batch_size = 10_000
    for batch_start in range(0, num_entries, batch_size):
        batch_end = min(batch_start + batch_size, num_entries)
        prices = generate_mean_reverting_stock_prices(start_price, batch_end - batch_start, volatility, mean_reversion_level)
        
        for i, open_price, high_price, low_price, close_price, volume in prices:
            writer.writerow([i, open_price, high_price, low_price, close_price, volume])

print("CSV file 'large_dataset.csv' created successfully!")




# Testing
#1 Dataset: largedataset
# Current directory: /Users/harrysingh/Library/Mobile Documents/com~apple~CloudDocs/Desktop/CS_101/uni/yr2/Term 3 2023/funpar/FunPar-Project/data-analyzer-ta
# Sequential Results:
# Dates: 30000000
# Closing Prices: 30000000
# EMA Values: 30000000
# Parallel Results:
# Dates: 30000000
# Closing Prices: 30000000
# EMA Values: 30000000
# Sequential processing took: 89.878101833s
# Parallel processing took: 77.100826334s

#2: largeDataset
# Current directory: /Users/harrysingh/Library/Mobile Documents/com~apple~CloudDocs/Desktop/CS_101/uni/yr2/Term 3 2023/funpar/FunPar-Project/data-analyzer-ta
# Sequential Results:
# Dates: 30000000
# Closing Prices: 30000000
# EMA Values: 30000000
# Parallel Results:
# Dates: 30000000
# Closing Prices: 30000000
# EMA Values: 30000000
# Sequential processing took: 90.636762833s
# Parallel processing took: 76.066746583s


#2: Dataset: superlarge_dataset
# Current directory: /Users/harrysingh/Library/Mobile Documents/com~apple~CloudDocs/Desktop/CS_101/uni/yr2/Term 3 2023/funpar/FunPar-Project/data-analyzer-ta
# Sequential Results:
# Dates: 50000000
# Closing Prices: 50000000
# EMA Values: 50000000
# Parallel Results:
# Dates: 50000000
# Closing Prices: 50000000
# EMA Values: 50000000
# Sequential processing took: 150.332246375s
# Parallel processing took: 129.129155416s







