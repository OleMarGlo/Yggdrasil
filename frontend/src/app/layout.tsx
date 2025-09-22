import type { Metadata } from 'next'
import './globals.css'
import NavBar from '@/components/navbar/nav'

export const metadata: Metadata = {
    title: 'Yggdrasil',
    description: 'Yggdrasil - Portfilio website',
}

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang='en'>
            <body className='flex flex-col h-full'>
                <div className='h-16'>
                    <NavBar />
                </div>
                <main className='flex flex-1 p-2'>
                    {children}
                </main>
            </body>
        </html>
    )
}
