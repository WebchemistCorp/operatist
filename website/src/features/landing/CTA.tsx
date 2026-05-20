import React, { useState } from 'react'
import { CheckCircle, Smartphone, Globe, Terminal } from 'lucide-react'
import { Button } from '@/components/ui/button'

const PRIMARY = '#2563eb'
const ACCENT = '#10b981'

const PLATFORMS = [
  { icon: Globe, label: '웹앱', desc: '지금 바로 브라우저에서' },
  { icon: Smartphone, label: '모바일', desc: 'iOS · Android 앱' },
  { icon: Terminal, label: 'CLI', desc: '터미널 파워유저용' },
]

export const CTA: React.FC = () => {
  const [email, setEmail] = useState('')
  const [submitted, setSubmitted] = useState(false)

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (!email) return
    setSubmitted(true)
    setEmail('')
  }

  return (
    <section id="cta" className="py-32 px-6">
      <div className="max-w-6xl mx-auto space-y-16">

        <div className="text-center space-y-4">
          <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: ACCENT }}>지금 시작하세요</p>
          <h2 className="text-4xl md:text-5xl font-bold text-slate-50" style={{ wordBreak: 'keep-all' }}>
            오늘부터
            <br />
            <span style={{ color: '#60a5fa' }}>본업에만 집중하세요</span>
          </h2>
          <p className="text-slate-400 text-lg max-w-xl mx-auto" style={{ wordBreak: 'keep-all' }}>
            얼리 액세스 대기자를 모집합니다. 이메일을 남기시면 출시 즉시 알려드립니다.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-5 max-w-3xl mx-auto">
          {PLATFORMS.map(({ icon: Icon, label, desc }) => (
            <div
              key={label}
              className="rounded-2xl p-6 border text-center space-y-3 transition-all duration-300 hover:scale-[1.02]"
              style={{ background: `${PRIMARY}0a`, borderColor: `${PRIMARY}22` }}
            >
              <span
                className="w-12 h-12 rounded-xl flex items-center justify-center mx-auto"
                style={{ background: `${PRIMARY}1a` }}
              >
                <Icon size={20} style={{ color: '#60a5fa' }} />
              </span>
              <p className="font-semibold text-slate-100">{label}</p>
              <p className="text-slate-500 text-sm">{desc}</p>
            </div>
          ))}
        </div>

        <div
          className="max-w-lg mx-auto rounded-2xl p-8 border space-y-6"
          style={{ background: `${ACCENT}08`, borderColor: `${ACCENT}22` }}
        >
          {submitted ? (
            <div className="text-center space-y-3">
              <CheckCircle size={36} className="mx-auto" style={{ color: ACCENT }} />
              <p className="text-slate-100 font-semibold text-lg">등록 완료!</p>
              <p className="text-slate-400 text-sm">출시 소식을 가장 먼저 알려드리겠습니다.</p>
            </div>
          ) : (
            <>
              <div className="space-y-2">
                <p className="font-semibold text-slate-100 text-lg">얼리 액세스 신청</p>
                <p className="text-slate-400 text-sm">얼리 액세스 사용자에게는 3개월 무료 혜택을 드립니다.</p>
              </div>
              <form onSubmit={handleSubmit} className="space-y-3">
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  placeholder="이메일 주소를 입력하세요"
                  className="w-full px-4 py-3 rounded-xl bg-slate-800/60 border border-slate-700/60 text-slate-100 placeholder-slate-500 text-sm focus:outline-none focus:border-blue-500/60 transition-colors"
                  required
                />
                <Button
                  type="submit"
                  className="w-full rounded-xl bg-blue-600 hover:bg-blue-500 text-white font-semibold py-3 h-auto shadow-[0_0_20px_rgba(37,99,235,0.3)] hover:scale-[1.01] active:scale-[0.99] transition-all duration-300"
                >
                  대기자 등록하기
                </Button>
              </form>
              <p className="text-slate-600 text-xs text-center">스팸 없음. 출시 소식만 전달드립니다.</p>
            </>
          )}
        </div>

      </div>
    </section>
  )
}
