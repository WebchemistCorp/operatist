import React, { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Menu, X } from 'lucide-react'
import { Button } from '@/components/ui/button'

const PRIMARY = '#f97316'

const NAV_ITEMS = [
  { label: '소개', href: '#about' },
  { label: '기능', href: '#features' },
]

export const Nav: React.FC = () => {
  const [scrolled, setScrolled] = useState(false)
  const [menuOpen, setMenuOpen] = useState(false)

  const handleLogoClick = (e: React.MouseEvent) => {
    e.preventDefault()
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 40)
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  return (
    <>
      <nav className="fixed top-4 left-1/2 -translate-x-1/2 z-50 transition-all duration-500 w-auto">
        <div
          className={`flex items-center gap-1 px-3 py-2 rounded-full border transition-all duration-500 ${
            scrolled
              ? 'bg-zinc-900/90 border-zinc-700/60 backdrop-blur-xl shadow-[0_8px_32px_rgba(0,0,0,0.4)]'
              : 'bg-zinc-900/60 border-zinc-800/40 backdrop-blur-md'
          }`}
        >
          <Button asChild variant="ghost" className="flex items-center gap-2 px-2 py-1 rounded-full hover:bg-zinc-800/60 mr-1 h-auto">
            <Link to="/" onClick={handleLogoClick}>
              <div
                className="w-5 h-5 rounded-full flex items-center justify-center"
                style={{ background: `${PRIMARY}22`, border: `1px solid ${PRIMARY}44` }}
              >
                <span className="text-[10px] font-bold" style={{ color: PRIMARY }}>O</span>
              </div>
              <span className="text-zinc-200 text-sm font-semibold tracking-tight hidden sm:block">Operatist</span>
            </Link>
          </Button>

          <div className="w-px h-4 bg-zinc-700/60 mx-1" />

          <div className="hidden md:flex items-center gap-0.5">
            {NAV_ITEMS.map((item) => (
              <Button key={item.label} asChild size="xs" variant="ghost" className="rounded-full text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800/60">
                <a href={item.href}>{item.label}</a>
              </Button>
            ))}
          </div>

          <div className="hidden md:flex items-center ml-1 shrink-0">
            <Button asChild size="xs" className="rounded-full font-semibold text-zinc-950" style={{ background: PRIMARY }}>
              <a href="https://github.com/WebchemistCorp/operatist/releases" target="_blank" rel="noreferrer">
                다운로드
              </a>
            </Button>
          </div>

          <button
            type="button"
            onClick={() => setMenuOpen(!menuOpen)}
            className="md:hidden w-8 h-8 flex items-center justify-center rounded-full text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800/60 transition-all duration-300"
            aria-label="메뉴"
          >
            {menuOpen ? <X size={18} /> : <Menu size={18} />}
          </button>
        </div>
      </nav>

      {menuOpen && (
        <div className="fixed inset-0 z-40 bg-zinc-950/95 backdrop-blur-3xl flex flex-col items-center justify-center gap-4 md:hidden">
          {NAV_ITEMS.map((item, i) => (
            <a
              key={item.label}
              href={item.href}
              onClick={() => setMenuOpen(false)}
              style={{ animationDelay: `${i * 60}ms` } as React.CSSProperties}
              className="reveal text-2xl font-bold text-zinc-300 transition-colors duration-300"
            >
              {item.label}
            </a>
          ))}
          <a
            href="https://github.com/WebchemistCorp/operatist/releases"
            target="_blank"
            rel="noreferrer"
            style={{ animationDelay: `${NAV_ITEMS.length * 60}ms`, background: PRIMARY } as React.CSSProperties}
            className="reveal mt-4 px-8 py-3 rounded-full text-zinc-950 font-semibold transition-all duration-300"
          >
            다운로드
          </a>
        </div>
      )}
    </>
  )
}
