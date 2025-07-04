declare module 'std' {
  import { File } from 'os';

  export interface EvalOptions {
    backtrace_barrier?: boolean;
  }

  export interface ErrorOptions {
    errorno: Error;
  }

  export interface URLGetOptions {
    binary?: boolean;
    full?: boolean;
  }

  export interface URLGetResponse {
    readonly response: string | null;
    readonly responseHeaders: string;
    readonly status: number;
  }

  export const SEEK_SET: number; // 0
  export const SEEK_CUR: number; // 1
  export const SEEK_END: number; // 2

  export const S_IFMT: number;
  export const S_IFIFO: number;
  export const S_IFCHR: number;
  export const S_IFDIR: number;
  export const S_IFBLK: number;
  export const S_IFREG: number;
  export const S_IFSOCK: number;
  export const S_IFLNK: number;
  export const S_ISGID: number;
  export const S_ISUID: number;

  export type Seek = unknown;
  export const enum Error {
    EACCES = 13,
    EBUSY = 16,
    EEXIST = 17,
    EINVAL = 22,
    EIO = 5,
    ENOENT = 2,
    ENOSPC = 28,
    ENOSYS = 38,
    EPERM = 1,
    EPIPE = 32,
  }

  export function exit(n: number): void;
  export function evalScript(script: string, options?: EvalOptions): void;
  export function loadScript(filename: string): void;
  export function loadFile(filename: string): string;
  export function open(
    filename: string,
    flags: unknown,
    errorObj?: ErrorOptions,
  ): File | null;
  export function popen(
    command: string,
    flags: unknown,
    errorObj?: ErrorOptions,
  ): File | null;
  export function fdopen(
    file: File,
    flags: unknown,
    errorObj?: ErrorOptions,
  ): File | null;
  export function tmpFile(errorObj?: ErrorOptions): File | null;
  export function puts(str: string): void;
  export function printf(fmt: string, ...args: any[]): void;
  export function sprintf(fmt: string, ...args: any[]): void;

  export function strerror(errorno: Error): string;
  export function gc(): void;
  export function getenv(name: string): any | undefined;
  export function setenv(name: string, value: any): void;
  export function unsetenv(name: string): void;
  export function getenviron(): { readonly [key: string]: string };
  export function urlGet(url: string): string;
  export function urlGet(
    url: string,
    options: { full?: false; binary: false },
  ): string;
  export function urlGet(
    url: string,
    options: { full?: false; binary: true },
  ): ArrayBuffer;
  export function urlGet(
    url: string,
    options: { full: true; binary?: false },
  ): URLGetResponse;
  export function urlGet(
    url: string,
    options: { full: true; binary?: false },
  ): ArrayBuffer;
  export function parseExtJSON(str: string): any;

  const _in: File;
  export { _in as in };
  export const err: File;
  export const out: File;
}
