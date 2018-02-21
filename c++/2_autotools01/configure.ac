#.ac stands for input to Autoconf
#Used to generate configure script
#AC_INIT([program_name],[version])
AC_INIT([sqroot_n],[1.0.0],[bug-report@domain.com])
AC_CONFIG_SRCDIR([sqroot.cpp]) 			# Safety: File must exist
AC_CONFIG_HEADERS([config.h]) 			# Make config header
AM_INIT_AUTOMAKE([foreign -Wall -Werror]) 	# Init Automake

#AC_PROG_CC				# Find & Probe C compiler
AC_PROG_CXX				# Find & Probe C++ compiler
AC_LANG([C++])

AC_CONFIG_FILES([Makefile]) 		# "Configure" creates Makefile 

#CFLAGS+=" -Werror -Wall -Wformat=2 -Wcast-qual -Wwrite-strings"
CXXFLAGS+=" -Werror -Wall"

#AC_OUTPUT is needed to write the output
AC_OUTPUT


# This really is a Bourne Shell script processed
# by the m4 macro processor, however it has the 
# pre-created definitions as seen above.

# it uses brackets [] as parameters. 
# also note that whitespaces on wrong places
# can cause problems.
# do not put whitespace before macro invocation "("
# No whitespaces after parameters -- before "," or ")" 

