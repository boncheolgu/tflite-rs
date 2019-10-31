! test of fftsg3d.f
!
      program main
      integer nmax, nmaxsqrt
      parameter (nmax = 128)
      parameter (nmaxsqrt = 16)
      integer ip(0 : nmaxsqrt + 1), n1, n2, n3, i, j
      real*8 a(0 : nmax - 1, 0 : nmax - 1, 0 : nmax - 1), 
     &    t(0 : 8 * nmax - 1), 
     &    w(0 : nmax * 3 / 2 - 1), err, errorcheck3d
!
      write (*, *) 'data length n1=? (n1 = power of 2) '
      read (*, *) n1
      write (*, *) 'data length n2=? (n2 = power of 2) '
      read (*, *) n2
      write (*, *) 'data length n3=? (n3 = power of 2) '
      read (*, *) n3
      ip(0) = 0
!
!   check of CDFT
      call putdata3d(nmax, nmax, n1, n2, n3, a)
      call cdft3d(nmax, nmax, n1, n2, n3, 1, a, t, ip, w)
      call cdft3d(nmax, nmax, n1, n2, n3, -1, a, t, ip, w)
      err = errorcheck3d(nmax, nmax, n1, n2, n3, 
     &    2.0d0 / n1 / n2 / n3, a)
      write (*, *) 'cdft3d err= ', err
!
!   check of RDFT
      call putdata3d(nmax, nmax, n1, n2, n3, a)
      call rdft3d(nmax, nmax, n1, n2, n3, 1, a, t, ip, w)
      call rdft3d(nmax, nmax, n1, n2, n3, -1, a, t, ip, w)
      err = errorcheck3d(nmax, nmax, n1, n2, n3, 
     &    2.0d0 / n1 / n2 / n3, a)
      write (*, *) 'rdft3d err= ', err
!
!   check of DDCT
      call putdata3d(nmax, nmax, n1, n2, n3, a)
      call ddct3d(nmax, nmax, n1, n2, n3, 1, a, t, ip, w)
      call ddct3d(nmax, nmax, n1, n2, n3, -1, a, t, ip, w)
      do j = 0, n2 - 1
          do i = 0, n1 - 1
              a(i, j, 0) = a(i, j, 0) * 0.5d0
          end do
      end do
      do j = 0, n3 - 1
          do i = 0, n1 - 1
              a(i, 0, j) = a(i, 0, j) * 0.5d0
          end do
          do i = 0, n2 - 1
              a(0, i, j) = a(0, i, j) * 0.5d0
          end do
      end do
      err = errorcheck3d(nmax, nmax, n1, n2, n3, 
     &    8.0d0 / n1 / n2 / n3, a)
      write (*, *) 'ddct3d err= ', err
!
!   check of DDST
      call putdata3d(nmax, nmax, n1, n2, n3, a)
      call ddst3d(nmax, nmax, n1, n2, n3, 1, a, t, ip, w)
      call ddst3d(nmax, nmax, n1, n2, n3, -1, a, t, ip, w)
      do j = 0, n2 - 1
          do i = 0, n1 - 1
              a(i, j, 0) = a(i, j, 0) * 0.5d0
          end do
      end do
      do j = 0, n3 - 1
          do i = 0, n1 - 1
              a(i, 0, j) = a(i, 0, j) * 0.5d0
          end do
          do i = 0, n2 - 1
              a(0, i, j) = a(0, i, j) * 0.5d0
          end do
      end do
      err = errorcheck3d(nmax, nmax, n1, n2, n3, 
     &    8.0d0 / n1 / n2 / n3, a)
      write (*, *) 'ddst3d err= ', err
!
      end
!
!
      subroutine putdata3d(n1max, n2max, n1, n2, n3, a)
      integer n1max, n2max, n1, n2, n3, j1, j2, j3, seed
      real*8 a(0 : n1max - 1, 0 : n2max - 1, 0 : *), drnd
      seed = 0
      do j3 = 0, n3 - 1
          do j2 = 0, n2 - 1
              do j1 = 0, n1 - 1
                  a(j1, j2, j3) = drnd(seed)
              end do
          end do
      end do
      end
!
!
      function errorcheck3d(n1max, n2max, n1, n2, n3, scale, a)
      integer n1max, n2max, n1, n2, n3, j1, j2, j3, seed
      real*8 scale, a(0 : n1max - 1, 0 : n2max - 1, 0 : *), 
     &    drnd, err, e, errorcheck3d
      err = 0
      seed = 0
      do j3 = 0, n3 - 1
          do j2 = 0, n2 - 1
              do j1 = 0, n1 - 1
                  e = drnd(seed) - a(j1, j2, j3) * scale
                  err = max(err, abs(e))
              end do
          end do
      end do
      errorcheck3d = err
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
