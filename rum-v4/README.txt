Adrian Sicaju Ruiz, Andre Qurioa

For assignment 6, we had receieved most of our help through Connor's discord sevrer by asking questions directly
to Connor and others students within the server. We had also receieved some help and guidance from Justin Watkins.


To enumerate any significant departures from our design was mainly where we planned to implement our 
counter to be able to calculate 50mill instructions in CPU seconds. We had some trouble implementing benchmarking
tools to recieve our output so we had used std::time::instant in between our while loop for our virutal machine
to be running and executing our instructions.

To describe our design, we have a main file that will read our input to our virtual machine and instructions 
and receive an output. 

Our instructions file called "instructs"that will operate the computations of our 14 instructions and 
semantic operators.


Our virtual machine file called "rum" that will contain its structure of containing the instructions
registers and segments. Essentially the operating system that will boot and execute all
our instructions by mapping and unmapping associated segment related register
operations and comparing them to their proper bit values for each opcode instruction
commands. 

Our bitpack file contains functions that will return a modified version of the unsigned
word. One that we used during our Arith assignment. This will help us by retrieving an
unsigned value from ‘word’ represented by width bits beginning at least a significant
bit. This was a bitpack file that was provided by our professor and peers and some modifications.


To explain how we were able to figure out how long it would take our UM to execute 50 million instructions, 
we used the "hello.um" file that was also provided to us for testing by our Professor to notice it contained
29 instructions, and with our time::instant, took about 2.70ms to run. So we did our math, 

2.70/29 = 0.09ms
0.09 * 50 million = 4,500,000ms
4,500,00ms = 4500 Seconds

If we did our math right, our UM would take 4500 seconds to execute a file of 50 million which is extremly slow
compared to what Professor Daniels is expecting. However, this calculation is soley based on our example because
we had trouble implementing our instant time loop outside our while loop and if we kept it inside, it would have taken
a very long time to still recieve our output so we made our calculations based on data we gathered.

We spent about 4-5 hours analyzing this assignment, spent about 4 hours preparing our design and spent a good 
7-8 hours spent trying to solve our problems after analysis. 