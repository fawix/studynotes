## This file will walk through a simple make and 
## needed improvements upon it divided in 4 parts
## At the end you'll have the basics for understanding
## and writing your own make files
## We will still see a bit more of advanced wizardry on 1_make03

## Feel free to comment and uncomment as  you go...
## Hint... you can delete the entire first row "#"
## of the part you are inspecting.. it will 
## expose the code and preserve the comments 
## (notice the "##") Just don't forget to comment
## the 'PART 4' below :)



## >>>>>>>>>>>>>>>> PART 1
##
##Most basic example of how we could compile this project
## Will recompile everything everytime...
##
##This is what the simplest version would look like:
##
#hello: hello.cpp hellofunc.cpp
#	g++ hello.cpp hellofunc.cpp -o hello
##
## When we have the the two lines above define our target "hello"
## and what defines the files that trigger this line on change.
## meaning if any of the two files change it will run the g++ command 
## for the hello target.
##
##
## Make with no arguments will execute the first rule in the file.
## Also IMPORTANT: there must be a tab at the beginning of any command
## make will not be happy if tab is not there.
##
## NOTE: you'll see an error like this:
## 	Makefile:124: *** missing separator.  Stop.
##		  ^
##		  line number
##
##
## Although this form works for small projects it's very ineficient since it will
## recompile all the files for every change.

## >>>>>>>>>>>>>>>> PART 2
## 
## READ PART 1 FIRST 
##
## We can also add a couple of flags to our targets...
## Examples:
#CXX=g++
#CXXFLAGS=-Wall
## If it were C it could be
##CC=gcc
##CFLAGS=-I
##
## These are special constants, that communicate to make how we want to compile the files
## in particular the macro CC and CXX contains the target compiler.
## and CXXFLAGS and CFLAGS are the flags for compilation.
## 
##And make use of the flags as such:
##
#hello: hello.cpp hellofunc.cpp
#	$(CXX) $(CXXFLAGS) hello.cpp hellofunc.cpp -o hello
##Res:	g++ -Wall hello.cpp hellofunc.cpp -o hello
##
## This form of makefile might be sufficient for small scale projects
## however, if we were to modify the hellomake.h file for example
## make would not recompile the cpp files, even though they need to be 
## recompiled to reflect the change.
##


## >>>>>>>>>>>>>>>> PART 3
##
## I' ll be re-using the flags we added on 
## part two so comment this out if testing to avoid conflict.
##
## As noted above, part 2 would not reflect changes made on
## *.h files.... 
##
#CXX=g++
#CXXFLAGS=-Wall
## We will put all the dependencies on a variable... 
#DEPS=hellomake.h
#
## ...and construct our compiling call as such...:
#%.o: %.cpp $(DEPS)
#	$(CXX) $(CXXFLAGS) -c $< -o $@ 
#
## ... and finally define our target:
#hello: hello.cpp hellofunc.cpp
#	$(CXX) $(CXXFLAGS) hello.cpp hellofunc.cpp -o hello
#
## This addition will first create a macro called DEPS,
## which is set to all *.h files that our *.cpp files depend on
## Thein we apply a rule with all files ending with the *.o suffix.
## The rule states:
## 	to generate a *.o file make needs to compile the *.cpp file using the 
## 	compiler defined in the CXX macro. The -c flag indicates that the compiler
## 	must generate the object file.
##
##
## The version above is for C++ for C we could have something like this:
## (Considering the flags of Part 2
## %.o: %.c $(DEPS)
##        $(CC) -c -o $@ $< $(CFLAGS) 
##
## Note that $< expands to the name of the first dependency. 
## Note that the $@ expands to the filename on the target (the value on the left side of ':')




## >>>>>>>>>>>>>>>> PART 4
#
# With what we just learned we can further simplify the target of our make to the following

CXX=g++
CXXFLAGS=-Wall
DEPS=hellomake.h
# We will declare our objects upfront:
OBJ=hello.o hellofunc.o

%.o: %.cpp $(DEPS)
	$(CXX) $(CXXFLAGS) -c $< -o $@ 

# and abstract away as much as possble here
hello: $(OBJ)
	$(CXX) $(CXXFLAGS) $^ -o $@
# Res:  g++ -Wall hello.cpp hellofunc.cpp -o hello
# Note:
# the '$^' is a special macro that represents the right side of the target (right of ':')
# and in this case is the value of OBJ.
#
# As seen above the $@ represents the left side of the target (in this case the value of hello)
#
# This Makefile is still rather simple and lacks some of the standard targets for Makefiles.
