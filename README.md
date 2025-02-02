Calculator Program
A command-line calculator that supports both single and multiple operations. The calculator can handle operations such as addition, subtraction, multiplication, division, modulus, and exponentiation.

Features:
Single operation calculation (e.g., 5 + 3).
Multiple operations in a sequence (e.g., 10 + 5 * 2).
Keeps track of the calculation history.
Operations Supported:
+ : Addition
- : Subtraction
* : Multiplication
/ : Division    
% : Modulus
^ : Exponentiation


Usage:
Run the program.
Choose between single or multiple operations.
Input the numbers and the operator(s).
See the result and optionally continue with more calculations.
The program keeps track of all results during the session.

example:
Single or multiple operation? (single/multiple): single
Enter first number: 10
Enter operator (+, -, *, /, %, ^): +
Enter second number: 5
Result: 15
Another calculation? (y/n): n
History: [15]


#after i run it inside a docker container 
 i first enter my vm and enter my working directory  

# firstly i have my dockerfile

#then i run this command to build it into the working directory 


```docker build -t calc .```

#the i modified my dockerfile and run this command and put app as the depencies>
```sudo docker run -it --rm --name my-container3 calc```

# after that i build it again so as to update the new modification and dependen>
```docker build -t calc .```

# after successfully build we run our container to se if its working
```sudo docker run -it --rm --name my-container3 calc```
# this command will take you to my calculator and prompt you to enter numbers 
