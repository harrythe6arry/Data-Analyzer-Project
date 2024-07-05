import csv
import random

# Function to generate mean-reverting stock prices
def generate_mean_reverting_stock_prices(start_price, total_entries, volatility, mean_reversion_level):
    prices = []
    current_price = start_price
    for _ in range(total_entries):
        # Calculate the change with mean reversion
        daily_change = random.uniform(-volatility, volatility)
        current_price += (mean_reversion_level - current_price) * 0.01 + daily_change
        
        open_price = current_price
        close_price = open_price * (1 + daily_change)
        high_price = max(open_price, close_price) * (1 + random.uniform(0, volatility))
        low_price = min(open_price, close_price) * (1 - random.uniform(0, volatility))
        volume = random.uniform(1000, 5000)
        prices.append((open_price, high_price, low_price, close_price, volume))
        current_price = close_price
    return prices

# Generate a large dataset
num_entries = 1000000 # Total number of entries
start_price = 100.0  # Starting price for the stock
volatility = 0.02  # Volatility factor to ensure consistent fluctuation
mean_reversion_level = 100.0  # Mean reversion level

prices = generate_mean_reverting_stock_prices(start_price, num_entries, volatility, mean_reversion_level)

# Create CSV file
with open('./examples/data/large_dataset.csv', mode='w', newline='') as file:
    writer = csv.writer(file)
    writer.writerow(['number', 'open', 'high', 'low', 'close', 'volume'])  # Write header

    for i, (open_price, high_price, low_price, close_price, volume) in enumerate(prices):
        writer.writerow([i, open_price, high_price, low_price, close_price, volume])

print("CSV file 'large_dataset.csv' created successfully!")
