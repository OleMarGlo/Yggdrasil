'use client'

import Link from 'next/link'
import { useState } from 'react'

type SpotlightLinkProps = {
    href: string
    position?: 'first' | 'last'
    children: React.ReactNode
}

export default function SpotlightLink({ href, children }: SpotlightLinkProps) {
    const [hovering, setHovering] = useState(false)
    const [bgPosition, setBgPosition] = useState({ x: 0, y: 0, width: 0, height: 0 })

    const handleMouseMove = (e: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
        const rect = e.currentTarget.getBoundingClientRect()
        setBgPosition({
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
            width: rect.width,
            height: rect.height,
        })
    }

    return (
        <Link
            href={href}
            className='relative rounded-2xl px-6 py-2 cursor-pointer border border-yggdrasil-200 hover:scale-103'
            onMouseEnter={() => setHovering(true)}
            onMouseLeave={() => setHovering(false)}
            onMouseMove={handleMouseMove}
            style={{
                background: hovering
                    ? `radial-gradient(circle 80px at ${bgPosition.x}px ${bgPosition.y}px, var(--color-yggdrasil-400), transparent 80%)`
                    : 'transparent',
                transform: hovering
                    ? `perspective(400px) rotateX(${((bgPosition.y - bgPosition.height / 2) / 20).toFixed(2)}deg) 
                        rotateY(${(-(bgPosition.x - bgPosition.width / 2) / 20).toFixed(2)}deg)`
                    : 'none',
                transition: 'transform 0.1s ease, background 0.2s ease',
            }}
        >
            {children}
        </Link>
    )
}
