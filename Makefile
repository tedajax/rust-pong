TARGET		= pong
LIBDIR		= lib
SRCDIR		= src
SOURCES		= main.rs
rm		= rm -f

all:
	rustc -L $(LIBDIR) $(SRCDIR)/$(SOURCES) -o $(TARGET) -g

run: all
	./$(TARGET)

clean:
	@$(rm) $(TARGET)