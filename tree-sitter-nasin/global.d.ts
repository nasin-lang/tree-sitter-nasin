type Rule = { __opaque?: "Rule" } | string | RegExp
type Grammar = { __opaque?: "Grammar" }
type RuleFunc<K extends string> = ($: Record<K, Rule>) => Rule
type RuleListFunc<K extends string> = ($: Record<K, Rule>) => Rule[]
type RuleMatrixFunc<K extends string> = ($: Record<K, Rule>) => Rule[][]
type GrammarDef<Rules extends string, Externals extends string> = {
    name: string
    rules: Partial<Record<Rules, RuleFunc<NoInfer<Rules | Externals>>>>
    word?: RuleFunc<NoInfer<Rules | Externals>>
    extras?: RuleListFunc<NoInfer<Rules | Externals>>
    inline?: RuleListFunc<NoInfer<Rules>>
    conflicts?: RuleMatrixFunc<NoInfer<Rules | Externals>>
    precedences?: RuleMatrixFunc<NoInfer<Rules | Externals>>
    externals?: RuleListFunc<Externals>
}

function grammar<Rules extends string>(def: GrammarDef<Rules, never>): Grammar
function grammar<Rules extends string, Externals extends string>(
    def: GrammarDef<Rules, Externals>,
): Grammar
function seq(...rules: Rule[]): Rule
function choice(...rules: Rule[]): Rule
function optional(rule: Rule): Rule
function repeat(rule: Rule): Rule
function repeat1(rule: Rule): Rule
function field(name: string, rule: Rule): Rule
function alias(rule: Rule, name: string): Rule
const prec: {
    (level: number, rule: Rule): Rule
    left(rule: Rule): Rule
    left(level: number, rule: Rule): Rule
    right(rule: Rule): Rule
    right(level: number, rule: Rule): Rule
    dynamic(level: number, rule: Rule): Rule
}
const token: {
    (rule: Rule): Rule
    immediate(rule: Rule): Rule
}
