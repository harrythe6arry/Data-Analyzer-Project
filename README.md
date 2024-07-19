# FunPar-Project Data Analyzer
This is my project for data analyzer. List Indicators that will covered in this Project is EMA. I will make the indicators read the CSV file in parallel with large datasets. Moreover, this project cover reading the parallel processes and the data is read in parallel. 

Aside from the parallelism, I've also added a python CSV file generator. This generator will generate a CSV file with a specified number of rows and columns. The data in the CSV file will be random numbers using mean reversion level to average the stock price. 

The output of the program would be in EMA values, closing price, and date.


# Run the Project
``
cargo run --bin "data-analyzer-ta"
``


# Benchmarking Process
The way I use to benchmark is I use time to measure the time it takes to run the program for the parallel function reading and the sequential function. I will run the program 10 times and average the time between sequential and parallel for **large datasets** and **superLargeDataset**.

## Benchmarking Result After Averaging 

### SuperLargeDataset (4.98 GB) Parallel and Sequential CSV reading 
The average processing times for the super large dataset are as follows:

- Average Sequential Processing Time: 153.810 seconds
- Average Parallel Processing Time: 131.856 seconds
Parallel processing is faster on average compared to sequential processing. 

### LargeDataSet (2.93)

The average processing times for the large dataset are as follows:

- Average Sequential Processing Time: 90.737 seconds
- Average Parallel Processing Time: 77.605 seconds
Parallel processing is faster on average compared to sequential processing.  


# Why is it faster?
The parallel processing is faster because it reads the data in parallel. This means that the program reads the data in chunks and processes the data in parallel. This allows the program to read the data faster compared to sequential processing.

# Addtional Resources for the project (BenchMarking)

LargeDataset(2.93 GB)

#1 Testing
Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.321093958s
Parallel processing took: 77.106426625s

#2 Testing

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.471153209s
Parallel processing took: 77.424025125s
 *  Terminal will be reused by tasks, press any key to close it. 

#3 Testing

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 91.264401917s
Parallel processing took: 76.896187709s

#4

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 91.328878666s
Parallel processing took: 78.235242875s

#5

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.164060208s
Parallel processing took: 76.903113541s

#6

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.9307495s
Parallel processing took: 77.71689325s

#7

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.798281416s
Parallel processing took: 78.068434083s

#8

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.820678667s
Parallel processing took: 78.07678375s

#9

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.72315125s
Parallel processing took: 77.901777042s

#10

Sequential Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Parallel Results:
Dates: 30000000
Closing Prices: 30000000
EMA Values: 30000000
Sequential processing took: 90.548244292s
Parallel processing took: 77.725023875s

BenchMarking for superLarge Dataset (4.89 GB)

#1 

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 152.110576375s
Parallel processing took: 131.045042541s

#2	

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 154.824314791s
Parallel processing took: 127.754043459s

#3

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 153.167904458s
Parallel processing took: 134.156367459s

#4

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 153.65825275s
Parallel processing took: 131.537559584s

#5

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 152.032954417s
Parallel processing took: 129.039591708s

#6

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 153.455752417s
Parallel processing took: 129.797765959s#7

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 159.018414667s
Parallel processing took: 128.861311291s

#8

Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 155.901902375s
Parallel processing took: 129.03924525s

#9

Reading and processing the CSV file in parallel...
Reading and processing the CSV file sequentially...
Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 151.799729458s
Parallel processing took: 148.371285083s

#10

Reading and processing the CSV file in parallel...
Reading and processing the CSV file sequentially...
Sequential Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Parallel Results:
Dates: 50000000
Closing Prices: 50000000
EMA Values: 50000000
Sequential processing took: 152.126691208s
Parallel processing took: 128.960298208s








