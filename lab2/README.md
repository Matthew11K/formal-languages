# Лабораторная работа №1

R=((bba∣a)b∣abab*(bab)*)*.

## ДКА

```dot
digraph DFA {
    rankdir=LR;
    "" [shape=none,label=""];
    "" -> q0;
    node [shape=doublecircle]; q0 q1 q2 q3 q4 q5 q6;
    node [shape=circle]; q7 q8 q9 q10 q11 q12 q13;
    q_dead [shape=circle,label="q14"];
    
    q0 -> q9 [label="a"];
    q0 -> q11 [label="b"];
    q1 -> q9 [label="a"];
    q1 -> q13 [label="b"];
    q2 -> q4 [label="a"];
    q2 -> q11 [label="b"];
    q3 -> q4 [label="a"];
    q3 -> q13 [label="b"];
    q4 -> q9 [label="a"];
    q4 -> q6 [label="b"];
    q5 -> q10 [label="a"];
    q5 -> q5 [label="b"];
    q6 -> q4 [label="a"];
    q6 -> q5 [label="b"];
    q7 -> q_dead [label="a"];
    q7 -> q1 [label="b"];
    q8 -> q_dead [label="a"];
    q8 -> q0 [label="b"];
    q9 -> q_dead [label="a"];
    q9 -> q2 [label="b"];
    q10 -> q_dead [label="a"];
    q10 -> q3 [label="b"];
    q11 -> q_dead [label="a"];
    q11 -> q12 [label="b"];
    q12 -> q8 [label="a"];
    q12 -> q_dead [label="b"];
    q13 -> q7 [label="a"];
    q13 -> q12 [label="b"];
    q_dead -> q_dead [label="a,b"];
}

```

Алфавит: {a, b}
Начальное состояние: q_0
Конечные состояния: q_0-6

### Таблица классов эквивалентности
| Состояние/слово    | ε | a | ab | abbab | b | ba | bab | bbab |
|--------------------|---|---|----|-------|---|----|-----|------|
| (q0)  ε            | + | − | +  | −     | − | −  | −   | +    |
| (q1)  ababbabbab   | + | − | +  | −     | − | −  | +   | +    |
| (q2)  ab           | + | + | +  | +     | − | −  | −   | +    |
| (q3)  ababbab      | + | + | +  | +     | − | −  | +   | +    |
| (q4)  aba          | + | − | +  | −     | + | +  | +   | +    |
| (q5)  ababb        | + | − | +  | +     | + | −  | +   | +    |
| (q6)  abab         | + | + | +  | +     | + | −  | +   | +    |
| (q7)  ababbabba    | − | − | −  | −     | + | −  | +   | +    |
| (q8)  bba          | − | − | −  | −     | + | −  | +   | −    |
| (q9)  a            | − | − | −  | −     | + | +  | +   | −    |
| (q10) ababba       | − | − | −  | −     | + | +  | +   | +    |
| (q11) b            | − | − | −  | −     | − | −  | +   | −    |
| (q12) bb           | − | − | +  | −     | − | −  | −   | −    |
| (q13) ababbabb     | − | − | +  | +     | − | −  | +   | −    |
| (q14) aa (ловушка) | − | − | −  | −     | − | −  | −   | −    |

Все строки попарно различаются -> все состояния находятся в разных классах эквивалетности -> ДКА минимален.

## НКА
```dot
digraph NFA {
    rankdir=LR;
    node [shape=circle];

    start [shape=point];
    start -> q0;

    q0 [shape=doublecircle];
    q6 [shape=doublecircle];
    q9 [shape=doublecircle];

    q0 -> q1 [label="a"];
    q0 -> q4 [label="a"];
    q0 -> q2 [label="b"];

    q1 -> q0 [label="b"];

    q2 -> q3 [label="b"];

    q3 -> q1 [label="a"];

    q4 -> q5 [label="b"];

    q5 -> q6 [label="a"];

    q6 -> q6 [label="b"];
    q6 -> q7 [label="b"];

    q7 -> q8 [label="a"];

    q8 -> q9 [label="b"];

    q9 -> q7 [label="b"];

    q6 -> q0 [label="ε"];
    q9 -> q0 [label="ε"];
}

```

Алфавит: {a, b}
Начальное состояние: q_0
Конечные состояния: q_0,6,9