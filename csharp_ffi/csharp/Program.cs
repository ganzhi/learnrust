using System.Runtime.InteropServices;

// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World! {0}", add(3, 5));

[DllImport("../target/debug/csharp_ffi.dll")]
static extern long add(long a, long b);