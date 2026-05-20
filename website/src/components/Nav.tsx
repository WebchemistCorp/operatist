import React, { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Menu, X } from 'lucide-react'
import { Button } from '@/components/ui/button'

const PRIMARY = '#f97316'

const NAV_ITEMS = [
  { label: '소개', href: '#about' },
  { label: '기능', href: '#features' },
]

const ECOSYSTEM = [
  { id: 'D', name: 'Devist', color: '#0ea5e9', url: 'https://devist.net', current: false },
  { id: 'O', name: 'Operatist', color: '#f97316', url: 'https://operatist.net', current: true },
  { id: 'C', name: 'Curatist', color: '#8b5cf6', url: 'https://curatist.net', current: false },
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

          <div className="w-px h-4 bg-zinc-700/60 mx-1 hidden md:block" />

          <div className="hidden md:flex items-center gap-0.5">
            {ECOSYSTEM.map(e => (
              e.current ? (
                <span
                  key={e.id}
                  className="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium"
                  style={{ background: `${e.color}18`, color: e.color }}
                >
                  <span className="w-1.5 h-1.5 rounded-full" style={{ background: e.color }} />
                  {e.name}
                </span>
              ) : (
                <a
                  key={e.id}
                  href={e.url}
                  className="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[11px] font-medium text-zinc-500 hover:text-zinc-300 hover:bg-zinc-800/60 transition-all duration-200"
                >
                  <span className="w-1.5 h-1.5 rounded-full" style={{ background: e.color }} />
                  {e.name}
                </a>
              )
            ))}
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
          <div className="reveal flex flex-col items-center gap-2 mt-6 w-full max-w-xs" style={{ animationDelay: `${NAV_ITEMS.length * 60}ms` } as React.CSSProperties}>
            <p className="text-[11px] text-zinc-600 uppercase tracking-[0.15em] font-medium mb-1">Asurada System</p>
            {ECOSYSTEM.map(e => (
              e.current ? (
                <span
                  key={e.id}
                  className="w-full flex items-center gap-3 px-5 py-3 rounded-2xl text-sm font-semibold"
                  style={{ background: `${e.color}18`, border: `1px solid ${e.color}33`, color: e.color }}
                >
                  <span className="w-2 h-2 rounded-full" style={{ background: e.color }} />
                  {e.name}
                  <span className="ml-auto text-[10px] opacity-60">현재</span>
                </span>
              ) : (
                <a
                  key={e.id}
                  href={e.url}
                  onClick={() => setMenuOpen(false)}
                  className="w-full flex items-center gap-3 px-5 py-3 rounded-2xl text-sm font-medium text-zinc-400 hover:text-zinc-200 transition-all duration-200"
                  style={{ background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)' }}
                >
                  <span className="w-2 h-2 rounded-full" style={{ background: e.color }} />
                  {e.name}
                </a>
              )
            ))}
          </div>
        </div>
      )}
    </>
  )
}
