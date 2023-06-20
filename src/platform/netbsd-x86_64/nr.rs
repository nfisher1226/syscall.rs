// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! System call numbers for x86-64 NetBSD.

pub const SYSCALL: usize = 0;
pub const EXIT: usize = 1;
pub const FORK: usize = 2;
pub const READ: usize = 3;
pub const WRITE: usize = 4;
pub const OPEN: usize = 5;
pub const CLOSE: usize = 6;
pub const COMPAT_50_WAIT4: usize = 7;
pub const COMPAT_43_OCREAT: usize = 8;
pub const LINK: usize = 9;
pub const UNLINK: usize = 10;
pub const CHDIR: usize = 12;
pub const FCHDIR: usize = 13;
pub const COMPAT_50_MKNOD: usize = 14;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 16;
pub const BREAK: usize = 17;
pub const COMPAT_20_GETFSSTAT: usize = 18;
pub const COMPAT_43_OLSEEK: usize = 19;
pub const GETPID: usize = 20;
pub const COMPAT_40_MOUNT: usize = 21;
pub const UNMOUNT: usize = 22;
pub const SETUID: usize = 23;
pub const GETUID: usize = 24;
pub const GETEUID: usize = 25;
pub const PTRACE: usize = 26;
pub const RECVMSG: usize = 27;
pub const SENDMSG: usize = 28;
pub const RECVFROM: usize = 29;
pub const ACCEPT: usize = 30;
pub const GETPEERNAME: usize = 31;
pub const GETSOCKNAME: usize = 32;
pub const ACCESS: usize = 33;
pub const CHFLAGS: usize = 34;
pub const FCHFLAGS: usize = 35;
pub const SYNC: usize = 36;
pub const KILL: usize = 37;
pub const COMPAT_43_STAT43: usize = 38;
pub const GETPPID: usize = 39;
pub const COMPAT_43_LSTAT43: usize = 40;
pub const DUP: usize = 41;
pub const PIPE: usize = 42;
pub const GETEGID: usize = 43;
pub const PROFIL: usize = 44;
pub const KTRACE: usize = 45;
pub const COMPAT_13_SIGACTION13: usize = 46;
pub const GETGID: usize = 47;
pub const COMPAT_13_SIGPROCMASK13: usize = 48;
pub const __GETLOGIN: usize = 49;
pub const __SETLOGIN: usize = 50;
pub const ACCT: usize = 51;
pub const COMPAT_13_SIGPENDING13: usize = 52;
pub const COMPAT_13_SIGALTSTACK13: usize = 53;
pub const IOCTL: usize = 54;
pub const COMPAT_12_OREBOOT: usize = 55;
pub const REVOKE: usize = 56;
pub const SYMLINK: usize = 57;
pub const READLINK: usize = 58;
pub const EXECVE: usize = 59;
pub const UMASK: usize = 60;
pub const CHROOT: usize = 61;
pub const COMPAT_43_FSTAT43: usize = 62;
pub const COMPAT_43_OGETKERNINFO: usize = 63;
pub const COMPAT_43_OGETPAGESIZE: usize = 64;
pub const COMPAT_12_MSYNC: usize = 65;
pub const VFORK: usize = 66;
pub const COMPAT_43_OMMAP: usize = 71;
pub const VADVISE: usize = 72;
pub const MUNMAP: usize = 73;
pub const MPROTECT: usize = 74;
pub const MADVISE: usize = 75;
pub const MINCORE: usize = 78;
pub const GETGROUPS: usize = 79;
pub const SETGROUPS: usize = 80;
pub const GETPGRP: usize = 81;
pub const SETPGID: usize = 82;
pub const COMPAT_50_SETITIMER: usize = 83;
pub const COMPAT_43_OWAIT: usize = 84;
pub const COMPAT_12_OSWAPON: usize = 85;
pub const COMPAT_50_GETITIMER: usize = 86;
pub const COMPAT_43_OGETHOSTNAME: usize = 87;
pub const COMPAT_43_OSETHOSTNAME: usize = 88;
pub const COMPAT_43_OGETDTABLESIZE: usize = 89;
pub const DUP2: usize = 90;
pub const FCNTL: usize = 92;
pub const COMPAT_50_SELECT: usize = 93;
pub const FSYNC: usize = 95;
pub const SETPRIORITY: usize = 96;
pub const COMPAT_30_SOCKET: usize = 97;
pub const CONNECT: usize = 98;
pub const COMPAT_43_OACCEPT: usize = 99;
pub const GETPRIORITY: usize = 100;
pub const COMPAT_43_OSEND: usize = 101;
pub const COMPAT_43_ORECV: usize = 102;
pub const COMPAT_13_SIGRETURN13: usize = 103;
pub const BIND: usize = 104;
pub const SETSOCKOPT: usize = 105;
pub const LISTEN: usize = 106;
pub const COMPAT_43_OSIGVEC: usize = 108;
pub const COMPAT_43_OSIGBLOCK: usize = 109;
pub const COMPAT_43_OSIGSETMASK: usize = 110;
pub const COMPAT_13_SIGSUSPEND13: usize = 111;
pub const COMPAT_43_OSIGSTACK: usize = 112;
pub const COMPAT_43_ORECVMSG: usize = 113;
pub const COMPAT_43_OSENDMSG: usize = 114;
pub const COMPAT_50_GETTIMEOFDAY: usize = 116;
pub const COMPAT_50_GETRUSAGE: usize = 117;
pub const GETSOCKOPT: usize = 118;
pub const READV: usize = 120;
pub const WRITEV: usize = 121;
pub const COMPAT_50_SETTIMEOFDAY: usize = 122;
pub const FCHOWN: usize = 123;
pub const FCHMOD: usize = 124;
pub const COMPAT_43_ORECVFROM: usize = 125;
pub const SETREUID: usize = 126;
pub const SETREGID: usize = 127;
pub const RENAME: usize = 128;
pub const COMPAT_43_OTRUNCATE: usize = 129;
pub const COMPAT_43_OFTRUNCATE: usize = 130;
pub const FLOCK: usize = 131;
pub const MKFIFO: usize = 132;
pub const SENDTO: usize = 133;
pub const SHUTDOWN: usize = 134;
pub const SOCKETPAIR: usize = 135;
pub const MKDIR: usize = 136;
pub const RMDIR: usize = 137;
pub const COMPAT_50_UTIMES: usize = 138;
pub const COMPAT_50_ADJTIME: usize = 140;
pub const COMPAT_43_OGETPEERNAME: usize = 141;
pub const COMPAT_43_OGETHOSTID: usize = 142;
pub const COMPAT_43_OSETHOSTID: usize = 143;
pub const COMPAT_43_OGETRLIMIT: usize = 144;
pub const COMPAT_43_OSETRLIMIT: usize = 145;
pub const COMPAT_43_OKILLPG: usize = 146;
pub const SETSID: usize = 147;
pub const COMPAT_50_QUOTACTL: usize = 148;
pub const COMPAT_43_OQUOTA: usize = 149;
pub const COMPAT_43_OGETSOCKNAME: usize = 150;
pub const NFSSVC: usize = 155;
pub const COMPAT_43_OGETDIRENTRIES: usize = 156;
pub const COMPAT_20_STATFS: usize = 157;
pub const COMPAT_20_FSTATFS: usize = 158;
pub const COMPAT_30_GETFH: usize = 161;
pub const COMPAT_09_OGETDOMAINNAME: usize = 162;
pub const COMPAT_09_OSETDOMAINNAME: usize = 163;
pub const COMPAT_09_OUNAME: usize = 164;
pub const SYSARCH: usize = 165;
pub const COMPAT_10_OSEMSYS: usize = 169;
pub const COMPAT_10_OMSGSYS: usize = 170;
pub const COMPAT_10_OSHMSYS: usize = 171;
pub const PREAD: usize = 173;
pub const PWRITE: usize = 174;
pub const COMPAT_30_NTP_GETTIME: usize = 175;
pub const NTP_ADJTIME: usize = 176;
pub const SETGID: usize = 181;
pub const SETEGID: usize = 182;
pub const SETEUID: usize = 183;
pub const LFS_BMAPV: usize = 184;
pub const LFS_MARKV: usize = 185;
pub const LFS_SEGCLEAN: usize = 186;
pub const COMPAT_50_LFS_SEGWAIT: usize = 187;
pub const COMPAT_12_STAT12: usize = 188;
pub const COMPAT_12_FSTAT12: usize = 189;
pub const COMPAT_12_LSTAT12: usize = 190;
pub const PATHCONF: usize = 191;
pub const FPATHCONF: usize = 192;
pub const GETSOCKOPT2: usize = 193;
pub const GETRLIMIT: usize = 194;
pub const SETRLIMIT: usize = 195;
pub const COMPAT_12_GETDIRENTRIES: usize = 196;
pub const MMAP: usize = 197;
pub const __SYSCALL: usize = 198;
pub const LSEEK: usize = 199;
pub const TRUNCATE: usize = 200;
pub const FTRUNCATE: usize = 201;
pub const __SYSCTL: usize = 202;
pub const MLOCK: usize = 203;
pub const MUNLOCK: usize = 204;
pub const UNDELETE: usize = 205;
pub const COMPAT_50_FUTIMES: usize = 206;
pub const GETPGID: usize = 207;
pub const REBOOT: usize = 208;
pub const POLL: usize = 209;
pub const AFSSYS: usize = 210;
pub const COMPAT_14___SEMCTL: usize = 220;
pub const SEMGET: usize = 221;
pub const SEMOP: usize = 222;
pub const SEMCONFIG: usize = 223;
pub const COMPAT_14_MSGCTL: usize = 224;
pub const MSGGET: usize = 225;
pub const MSGSND: usize = 226;
pub const MSGRCV: usize = 227;
pub const SHMAT: usize = 228;
pub const COMPAT_14_SHMCTL: usize = 229;
pub const SHMDT: usize = 230;
pub const SHMGET: usize = 231;
pub const COMPAT_50_CLOCK_GETTIME: usize = 232;
pub const COMPAT_50_CLOCK_SETTIME: usize = 233;
pub const COMPAT_50_CLOCK_GETRES: usize = 234;
pub const TIMER_CREATE: usize = 235;
pub const TIMER_DELETE: usize = 236;
pub const COMPAT_50_TIMER_SETTIME: usize = 237;
pub const COMPAT_50_TIMER_GETTIME: usize = 238;
pub const TIMER_GETOVERRUN: usize = 239;
pub const COMPAT_50_NANOSLEEP: usize = 240;
pub const FDATASYNC: usize = 241;
pub const MLOCKALL: usize = 242;
pub const MUNLOCKALL: usize = 243;
pub const COMPAT_50___SIGTIMEDWAIT: usize = 244;
pub const SIGQUEUEINFO: usize = 245;
pub const MODCTL: usize = 246;
pub const _KSEM_INIT: usize = 247;
pub const _KSEM_OPEN: usize = 248;
pub const _KSEM_UNLINK: usize = 249;
pub const _KSEM_CLOSE: usize = 250;
pub const _KSEM_POST: usize = 251;
pub const _KSEM_WAIT: usize = 252;
pub const _KSEM_TRYWAIT: usize = 253;
pub const _KSEM_GETVALUE: usize = 254;
pub const _KSEM_DESTROY: usize = 255;
pub const _KSEM_TIMEDWAIT: usize = 256;
pub const MQ_OPEN: usize = 257;
pub const MQ_CLOSE: usize = 258;
pub const MQ_UNLINK: usize = 259;
pub const MQ_GETATTR: usize = 260;
pub const MQ_SETATTR: usize = 261;
pub const MQ_NOTIFY: usize = 262;
pub const MQ_SEND: usize = 263;
pub const MQ_RECEIVE: usize = 264;
pub const COMPAT_50_MQ_TIMEDSEND: usize = 265;
pub const COMPAT_50_MQ_TIMEDRECEIVE: usize = 266;
pub const __POSIX_RENAME: usize = 270;
pub const SWAPCTL: usize = 271;
pub const COMPAT_30_GETDENTS: usize = 272;
pub const MINHERIT: usize = 273;
pub const LCHMOD: usize = 274;
pub const LCHOWN: usize = 275;
pub const COMPAT_50_LUTIMES: usize = 276;
pub const __MSYNC13: usize = 277;
pub const COMPAT_30___STAT13: usize = 278;
pub const COMPAT_30___FSTAT13: usize = 279;
pub const COMPAT_30___LSTAT13: usize = 280;
pub const __SIGALTSTACK14: usize = 281;
pub const __VFORK14: usize = 282;
pub const __POSIX_CHOWN: usize = 283;
pub const __POSIX_FCHOWN: usize = 284;
pub const __POSIX_LCHOWN: usize = 285;
pub const GETSID: usize = 286;
pub const __CLONE: usize = 287;
pub const FKTRACE: usize = 288;
pub const PREADV: usize = 289;
pub const PWRITEV: usize = 290;
pub const COMPAT_16___SIGACTION14: usize = 291;
pub const __SIGPENDING14: usize = 292;
pub const __SIGPROCMASK14: usize = 293;
pub const __SIGSUSPEND14: usize = 294;
pub const COMPAT_16___SIGRETURN14: usize = 295;
pub const __GETCWD: usize = 296;
pub const FCHROOT: usize = 297;
pub const COMPAT_30_FHOPEN: usize = 298;
pub const COMPAT_30_FHSTAT: usize = 299;
pub const COMPAT_20_FHSTATFS: usize = 300;
pub const COMPAT_50_____SEMCTL13: usize = 301;
pub const COMPAT_50___MSGCTL13: usize = 302;
pub const COMPAT_50___SHMCTL13: usize = 303;
pub const LCHFLAGS: usize = 304;
pub const ISSETUGID: usize = 305;
pub const UTRACE: usize = 306;
pub const GETCONTEXT: usize = 307;
pub const SETCONTEXT: usize = 308;
pub const _LWP_CREATE: usize = 309;
pub const _LWP_EXIT: usize = 310;
pub const _LWP_SELF: usize = 311;
pub const _LWP_WAIT: usize = 312;
pub const _LWP_SUSPEND: usize = 313;
pub const _LWP_CONTINUE: usize = 314;
pub const _LWP_WAKEUP: usize = 315;
pub const _LWP_GETPRIVATE: usize = 316;
pub const _LWP_SETPRIVATE: usize = 317;
pub const _LWP_KILL: usize = 318;
pub const _LWP_DETACH: usize = 319;
pub const COMPAT_50__LWP_PARK: usize = 320;
pub const _LWP_UNPARK: usize = 321;
pub const _LWP_UNPARK_ALL: usize = 322;
pub const _LWP_SETNAME: usize = 323;
pub const _LWP_GETNAME: usize = 324;
pub const _LWP_CTL: usize = 325;
pub const COMPAT_60_SA_REGISTER: usize = 330;
pub const COMPAT_60_SA_STACKS: usize = 331;
pub const COMPAT_60_SA_ENABLE: usize = 332;
pub const COMPAT_60_SA_SETCONCURRENCY: usize = 333;
pub const COMPAT_60_SA_YIELD: usize = 334;
pub const COMPAT_60_SA_PREEMPT: usize = 335;
pub const __SIGACTION_SIGTRAMP: usize = 340;
pub const RASCTL: usize = 343;
pub const KQUEUE: usize = 344;
pub const COMPAT_50_KEVENT: usize = 345;
pub const _SCHED_SETPARAM: usize = 346;
pub const _SCHED_GETPARAM: usize = 347;
pub const _SCHED_SETAFFINITY: usize = 348;
pub const _SCHED_GETAFFINITY: usize = 349;
pub const SCHED_YIELD: usize = 350;
pub const _SCHED_PROTECT: usize = 351;
pub const FSYNC_RANGE: usize = 354;
pub const UUIDGEN: usize = 355;
pub const GETVFSSTAT: usize = 356;
pub const STATVFS1: usize = 357;
pub const FSTATVFS1: usize = 358;
pub const COMPAT_30_FHSTATVFS1: usize = 359;
pub const EXTATTRCTL: usize = 360;
pub const EXTATTR_SET_FILE: usize = 361;
pub const EXTATTR_GET_FILE: usize = 362;
pub const EXTATTR_DELETE_FILE: usize = 363;
pub const EXTATTR_SET_FD: usize = 364;
pub const EXTATTR_GET_FD: usize = 365;
pub const EXTATTR_DELETE_FD: usize = 366;
pub const EXTATTR_SET_LINK: usize = 367;
pub const EXTATTR_GET_LINK: usize = 368;
pub const EXTATTR_DELETE_LINK: usize = 369;
pub const EXTATTR_LIST_FD: usize = 370;
pub const EXTATTR_LIST_FILE: usize = 371;
pub const EXTATTR_LIST_LINK: usize = 372;
pub const COMPAT_50_PSELECT: usize = 373;
pub const COMPAT_50_POLLTS: usize = 374;
pub const SETXATTR: usize = 375;
pub const LSETXATTR: usize = 376;
pub const FSETXATTR: usize = 377;
pub const GETXATTR: usize = 378;
pub const LGETXATTR: usize = 379;
pub const FGETXATTR: usize = 380;
pub const LISTXATTR: usize = 381;
pub const LLISTXATTR: usize = 382;
pub const FLISTXATTR: usize = 383;
pub const REMOVEXATTR: usize = 384;
pub const LREMOVEXATTR: usize = 385;
pub const FREMOVEXATTR: usize = 386;
pub const COMPAT_50___STAT30: usize = 387;
pub const COMPAT_50___FSTAT30: usize = 388;
pub const COMPAT_50___LSTAT30: usize = 389;
pub const __GETDENTS30: usize = 390;
pub const COMPAT_30___FHSTAT30: usize = 392;
pub const COMPAT_50___NTP_GETTIME30: usize = 393;
pub const __SOCKET30: usize = 394;
pub const __GETFH30: usize = 395;
pub const __FHOPEN40: usize = 396;
pub const __FHSTATVFS140: usize = 397;
pub const COMPAT_50___FHSTAT40: usize = 398;
pub const AIO_CANCEL: usize = 399;
pub const AIO_ERROR: usize = 400;
pub const AIO_FSYNC: usize = 401;
pub const AIO_READ: usize = 402;
pub const AIO_RETURN: usize = 403;
pub const COMPAT_50_AIO_SUSPEND: usize = 404;
pub const AIO_WRITE: usize = 405;
pub const LIO_LISTIO: usize = 406;
pub const __MOUNT50: usize = 410;
pub const MREMAP: usize = 411;
pub const PSET_CREATE: usize = 412;
pub const PSET_DESTROY: usize = 413;
pub const PSET_ASSIGN: usize = 414;
pub const _PSET_BIND: usize = 415;
pub const __POSIX_FADVISE50: usize = 416;
pub const __SELECT50: usize = 417;
pub const __GETTIMEOFDAY50: usize = 418;
pub const __SETTIMEOFDAY50: usize = 419;
pub const __UTIMES50: usize = 420;
pub const __ADJTIME50: usize = 421;
pub const __LFS_SEGWAIT50: usize = 422;
pub const __FUTIMES50: usize = 423;
pub const __LUTIMES50: usize = 424;
pub const __SETITIMER50: usize = 425;
pub const __GETITIMER50: usize = 426;
pub const __CLOCK_GETTIME50: usize = 427;
pub const __CLOCK_SETTIME50: usize = 428;
pub const __CLOCK_GETRES50: usize = 429;
pub const __NANOSLEEP50: usize = 430;
pub const ____SIGTIMEDWAIT50: usize = 431;
pub const __MQ_TIMEDSEND50: usize = 432;
pub const __MQ_TIMEDRECEIVE50: usize = 433;
pub const COMPAT_60__LWP_PARK: usize = 434;
pub const __KEVENT50: usize = 435;
pub const __PSELECT50: usize = 436;
pub const __POLLTS50: usize = 437;
pub const __AIO_SUSPEND50: usize = 438;
pub const __STAT50: usize = 439;
pub const __FSTAT50: usize = 440;
pub const __LSTAT50: usize = 441;
pub const ____SEMCTL50: usize = 442;
pub const __SHMCTL50: usize = 443;
pub const __MSGCTL50: usize = 444;
pub const __GETRUSAGE50: usize = 445;
pub const __TIMER_SETTIME50: usize = 446;
pub const __TIMER_GETTIME50: usize = 447;
pub const __NTP_GETTIME50: usize = 448;
pub const __WAIT450: usize = 449;
pub const __MKNOD50: usize = 450;
pub const __FHSTAT50: usize = 451;
pub const PIPE2: usize = 453;
pub const DUP3: usize = 454;
pub const KQUEUE1: usize = 455;
pub const PACCEPT: usize = 456;
pub const LINKAT: usize = 457;
pub const RENAMEAT: usize = 458;
pub const MKFIFOAT: usize = 459;
pub const MKNODAT: usize = 460;
pub const MKDIRAT: usize = 461;
pub const FACCESSAT: usize = 462;
pub const FCHMODAT: usize = 463;
pub const FCHOWNAT: usize = 464;
pub const FEXECVE: usize = 465;
pub const FSTATAT: usize = 466;
pub const UTIMENSAT: usize = 467;
pub const OPENAT: usize = 468;
pub const READLINKAT: usize = 469;
pub const SYMLINKAT: usize = 470;
pub const UNLINKAT: usize = 471;
pub const FUTIMENS: usize = 472;
pub const __QUOTACTL: usize = 473;
pub const POSIX_SPAWN: usize = 474;
pub const RECVMMSG: usize = 475;
pub const SENDMMSG: usize = 476;
pub const CLOCK_NANOSLEEP: usize = 477;
pub const ___LWP_PARK60: usize = 478;
pub const POSIX_FALLOCATE: usize = 479;
pub const FDISCARD: usize = 480;
pub const WAIT6: usize = 481;
pub const CLOCK_GETCPUCLOCKID2: usize = 482;
pub const MAXSYSCALL: usize = 483;
pub const NSYSENT: usize = 512;
