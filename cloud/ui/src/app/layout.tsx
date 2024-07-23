import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { config } from "../config";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: config.brand.name,
  description: config.brand.tagline,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
