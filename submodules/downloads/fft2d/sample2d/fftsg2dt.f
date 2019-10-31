! test of fftsg2d.f
!
      program main
      integer nmax, nmaxsqrt
      parameter (nmax = 1024)
      parameter (nmaxsqrt = 32)
      integer ip(0 : nmaxsqrt + 1), n1, n2, i
      real*8 a(0 : nmax - 1, 0 : nmax - 1), t(0 : 8 * nmax - 1), 
     &    w(0 : nmax * 3 / 2 - 1), err, errorcheck2d
!
      write (*, *) 'data length n1=? (n1 = power of 2) '
      read (*, *) n1
      write (*, *) 'data length n2=? (n2 = power of 2) '
      read (*, *) n2
      ip(0) = 0
!
!   check of CDFT
      call putdata2d(nmax, n1, n2, a)
      call cdft2d(nmax, n1, n2, 1, a, t, ip, w)
      call cdft2d(nmax, n1, n2, -1, a, t, ip, w)
      err = errorcheck2d(nmax, n1, n2, 2.0d0 / n1 / n2, a)
      write (*, *) 'cdft2d err= ', err
!
!   check of RDFT
      call putdata2d(nmax, n1, n2, a)
      call rdft2d(nmax, n1, n2, 1, a, t, ip, w)
      call rdft2d(nmax, n1, n2, -1, a, t, ip, w)
      err = errorcheck2d(nmax, n1, n2, 2.0d0 / n1 / n2, a)
      write (*, *) 'rdft2d err= ', err
!
!   check of DDCT
      call putdata2d(nmax, n1, n2, a)
      call ddct2d(nmax, n1, n2, 1, a, t, ip, w)
      call ddct2d(nmax, n1, n2, -1, a, t, ip, w)
      do i = 0, n1 - 1
          a(i, 0) = a(i, 0) * 0.5d0
      end do
      do i = 0, n2 - 1
          a(0, i) = a(0, i) * 0.5d0
      end do
      err = errorcheck2d(nmax, n1, n2, 4.0d0 / n1 / n2, a)
      write (*, *) 'ddct2d err= ', err
!
!   check of DDST
      call putdata2d(nmax, n1, n2, a)
      call ddst2d(nmax, n1, n2, 1, a, t, ip, w)
      call ddst2d(nmax, n1, n2, -1, a, t, ip, w)
      do i = 0, n1 - 1
          a(i, 0) = a(i, 0) * 0.5d0
      end do
      do i = 0, n2 - 1
          a(0, i) = a(0, i) * 0.5d0
      end do
      err = errorcheck2d(nmax, n1, n2, 4.0d0 / n1 / n2, a)
      write (*, *) 'ddst2d err= ', err
!
      end
!
!
      subroutine putdata2d(n1max, n1, n2, a)
      integer n1max, n1, n2, j1, j2, seed
      real*8 a(0 : n1max - 1, 0 : *), drnd
      seed = 0
      do j2 = 0, n2 - 1
          do j1 = 0, n1 - 1
              a(j1, j2) = drnd(seed)
          end do
      end do
      end
!
!
      function errorcheck2d(n1max, n1, n2, scale, a)
      integer n1max, n1, n2, j1, j2, seed
      real*8 scale, a(0 : n1max - 1, 0 : *), drnd, err, e, 
     &    errorcheck2d
      err = 0
      seed = 0
      do j2 = 0, n2 - 1
          do j1 = 0, n1 - 1
              e = drnd(seed) - a(j1, j2) * scale
              err = max(err, abs(e))
          end do
      end do
      errorcheck2d = err
      end
!
!
! random number generator, 0 <= drnd < 1
      real*8 function drnd(seed)
      integer seed
      seed = mod(seed * 7141 + 54773, 259200)
      drnd = seed * (1.0d0 / 259200)
      end
!
