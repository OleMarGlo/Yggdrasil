import Link from 'next/link'
import { FaGithub, FaLinkedin } from 'react-icons/fa'

export default function Footer() {
    return (
        <div className='flex p-2 justify-between'>
            <div className='flex'>
                <Link href='https://github.com/OleMarGlo' target='_blank'>
                    <FaGithub className='size-6' />
                </Link>
                <Link href='https://no.linkedin.com/in/ole-marius-glomsrud-0a677a354' target='_blank'>
                    <FaLinkedin className='size-6' />
                </Link>
            </div>
            <p className=''>&copy; Ole Marius Glomsrud. All rights reserved</p>
        </div>
    )
}