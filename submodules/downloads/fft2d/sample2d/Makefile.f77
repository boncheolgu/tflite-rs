# ---- for GNU g77 ----

F77 = g77

FFLAGS = -Wall

OFLAGS = -O2

# ---- for SUN WS f77 ----
#
#F77 = f77
#
#FFLAGS = 
#
#OFLAGS = -xO2




all: fft4f2dt_f fftsg2dt_f fftsg3dt_f


fft4f2dt_f : fft4f2dt_f.o fft4f2d_f.o
	$(F77) fft4f2dt_f.o fft4f2d_f.o -o fft4f2dt_f

fftsg2dt_f : fftsg2dt_f.o fftsg2d_f.o fftsg_f.o
	$(F77) fftsg2dt_f.o fftsg2d_f.o fftsg_f.o -o fftsg2dt_f

fftsg3dt_f : fftsg3dt_f.o fftsg3d_f.o fftsg_f.o
	$(F77) fftsg3dt_f.o fftsg3d_f.o fftsg_f.o -o fftsg3dt_f


fft4f2dt_f.o : fft4f2dt.f
	$(F77) $(FFLAGS) $(OFLAGS) -c fft4f2dt.f -o fft4f2dt_f.o

fftsg2dt_f.o : fftsg2dt.f
	$(F77) $(FFLAGS) $(OFLAGS) -c fftsg2dt.f -o fftsg2dt_f.o

fftsg3dt_f.o : fftsg3dt.f
	$(F77) $(FFLAGS) $(OFLAGS) -c fftsg3dt.f -o fftsg3dt_f.o


fft4f2d_f.o : ../fft4f2d.f
	$(F77) $(FFLAGS) $(OFLAGS) -c ../fft4f2d.f -o fft4f2d_f.o

fftsg2d_f.o : ../fftsg2d.f
	$(F77) $(FFLAGS) $(OFLAGS) -c ../fftsg2d.f -o fftsg2d_f.o

fftsg3d_f.o : ../fftsg3d.f
	$(F77) $(FFLAGS) $(OFLAGS) -c ../fftsg3d.f -o fftsg3d_f.o

fftsg_f.o : ../fftsg.f
	$(F77) $(FFLAGS) $(OFLAGS) -c ../fftsg.f -o fftsg_f.o




clean:
	rm -f *.o

