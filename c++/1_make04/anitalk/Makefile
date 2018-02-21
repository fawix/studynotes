appname := anitalk

IDIR=include
SDIR=src
BDIR=build
LDIR=libs


CXX=g++
CPPFLAGS=-I./$(IDIR)
CXXFLAGS=-Wall 
#LDLIBS=`pkg-config --libs opencv`

CPPFILES=$(shell find . -name "*.cpp")
OBJS=$(patsubst %.cpp, %.o, $(CPPFILES))

all: $(appname)
	@echo $(CPPFILES)
	@echo $(OBJS)

$(appname): $(objects)
	$(CXX) $(CXXFLAGS) $(LDFLAGS) -o $(appname) $(OBJS) $(LDLIBS)

#$(ODIR)/%.o: $(SDIR)/%.cpp $(DEPS)
#       $(CXX) $(CXXFLAGS) -c $< -o $@ $(CPPFLAGS)



depend: .depend

.depend: $(CPPFILES)
	rm -f ./.depend
	$(CXX) $(CXXFLAGS) -MM $^>>./.depend;

clean: 
	rm -f $(OBJS)

dist-clean: clean
	rm -f *~ .depend

include .depend




# Note... I haven't finished this Makefile yet
# Spending more time than I want on this really.
