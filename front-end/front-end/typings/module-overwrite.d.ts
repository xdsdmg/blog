declare module '*.less' {
  interface CssExports {
    [key: string]: string;
  }

  export const cssExports: CssExports;
  export default cssExports;
}
