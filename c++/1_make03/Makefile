## >>>>>>>>>>>>>>>> PART 5
#
# Now we will standardize and add some the neat make stuff (only the basic)
# for our hello project.
#
# Our project will have the following structure
# which is pretty standard nowadays.
# (although there is no standard really)
#
# project/
#   +
#   |
#   +----+ Makefile
#   |
#   +----+ obj/
#   |       +
#   |       +---> *.o files
#   |
#   +----+ src/
#   |       +
#   |       +--->  Source files (*.cpp & *.h)
#   |
#   +----+ include/
#   |       +
#   |       +---> Public Heads (*.h)
#   |
#   +----+ lib
#           +
#           +---> project libraries
#
# Note on include directory:
# Separating the .cpp of the .h file is not always a good solution. 
# Generally separate both of them when it is used as a library 
# (i.e.: public header in include and private header with the source code).
#
# At present this seem to be the most common project structure for 
# C++ Projects at open source community:
#
# bin/    binaries
# data/   data files
# lib/    libraries (may be libs/ as well)
# log/    program logs
# obj/    build object files
# src/    source files
# tmp/    temporary files
#
# In addition, one thing that's very common among Unix libraries is that 
# they are organized such that:
#
# ./         Makefile and configure scripts.
# ./src      General sources
# ./include  Header files that expose the public interface and are to be installed
# ./lib      Library build directory
# ./bin      Tools build directory
# ./tools    Tools sources
# ./test     Test suites that should be run during a `make test`
#
#

# Include Directory
IDIR=include
# Object Directory
ODIR=obj
# Library Directory
LDIR=lib
#Source Directory
SDIR=src

CXX=g++
CXXFLAGS=-Wall
CPPFLAGS=-I./$(IDIR)

# Listing our libraries
# this is the math library... just for fun
# our hello world actually would have this empty.
LIBS=-lm

# Our dependencies
# We'll pretend our hellomake.h is a public header
# and will be in include dir for demonstration
_DEPS=hellomake.h
DEPS=$(patsubst %, $(IDIR)/%, $(_DEPS))
# 
# patsubst will find whitepace-separated words in text that match pattern and replaces with replacement. 
# as such: $(patsubst pattern, replacement, text)
# the result of the above is 
# 	% -> represents our dependency list "hellomake.h" in our case.
#	$(IDIR)/% -> represents our path include/hellomake.h 
#	$(_DEPS) -> the text we want to replace. 
#
# See more: https://www.gnu.org/software/make/manual/html_node/Text-Functions.html

#Similar to lib this will place objects in our obj folder.
_OBJ=hello.o hellofunc.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: $(SDIR)/%.cpp $(DEPS)
	$(CXX) $(CXXFLAGS) -c $< -o $@ $(CPPFLAGS)

# and abstract away as much as possble here
hello: $(OBJ)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $^ -o $@

# This avoids messing up if a file
# named clean exists in the directory.
.PHONY: clean

clean:
	rm -f $(ODIR)/*.o

# Some caveats:
# Notice that this is a recursive Makefile (in how it navigates the folder tree)
# and therefore it will get progressive slower as project grows.
# Check the more advanced setup at 1_make04
#
# There are sevearl other more advanced things tthis file won't handle and
# likely will require adjustemtns as teh project grows.
# Also it's not re-usable (not without modifications anyway).
#
# Also, with this format if obj folder didn't previously exist it will fail.
