import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Stellar Space Game',
  description: 'An exciting space exploration game built on the Stellar blockchain',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-gray-900 text-white min-h-screen`}>
        <nav className="bg-gray-800 p-4">
          <div className="container mx-auto flex justify-between items-center">
            <h1 className="text-2xl font-bold">Stellar Space Game</h1>
            <div className="flex space-x-4">
              <button className="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700">
                Connect Wallet
              </button>
            </div>
          </div>
        </nav>
        <main className="container mx-auto py-8 px-4">
          {children}
        </main>
      </body>
    </html>
  )
} 