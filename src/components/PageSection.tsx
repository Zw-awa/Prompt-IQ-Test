import type { ReactNode } from 'react'

type PageSectionProps = {
  title: string
  description?: string
  actions?: ReactNode
  children: ReactNode
  className?: string
}

export function PageSection({
  title,
  description,
  actions,
  children,
  className,
}: PageSectionProps) {
  const sectionClassName = ['page-section', className].filter(Boolean).join(' ')

  return (
    <section className={sectionClassName}>
      <header className="page-section__header">
        <div>
          <h2>{title}</h2>
          {description ? <p>{description}</p> : null}
        </div>
        {actions ? <div className="page-section__actions">{actions}</div> : null}
      </header>
      <div className="page-section__body">{children}</div>
    </section>
  )
}
