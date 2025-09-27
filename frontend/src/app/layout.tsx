import type { Metadata } from 'next'
import './globals.css'
import NavBar from '@/components/navbar/nav'
import Footer from '@/components/footer/footer'

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
            <body className='flex flex-col'>
                <div className='h-16'>
                    <NavBar />
                </div>
                <div className='max-w-7xl mx-auto bg-yggdrasil-900'>
                    <main className='flex flex-1 p-4'>
                        {children}
                    </main>
                    <div>
                        <Footer />
                    </div>
                </div>
            </body>
        </html>
    )
}
