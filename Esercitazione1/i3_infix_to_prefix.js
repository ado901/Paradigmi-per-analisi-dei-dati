/*
{} = opzionale, può esserci (una o infinite volte) come può non esserci.
('a' | 'b') = c'è sicuramente a o b.
'(' a ')' = c'è (a), quindi '(' legge la parentesi nel testo, non come il caso precedente che indica l'or.  
Grammatica:
expr = term {( '+' | '-' ) term}
term = factor {( '*' | '/' ) factor}
factor = '-' factor | '(' expr ')' | var | num
var = 'w' | 'x' | 'y' | 'z'
/*

/*
expr = term {( '+' | '-' ) term}
Una expr è sempre formata da un termine. Se dopo non c'è nulla, valutiamo il term per ottenere il risultato finale
dell'espressione. In alternativa, il termine può essere seguito da un '+' o un '-', con un altro termine. 
In questo caso valutiamo separatamente i due term e poi facciamo la somma/differenza tra i risultati ottenuti.
tok: è un oggetto della classe Tokenizer, definito sul testo dell'espressione
    e su una regex per filtrare i diversi token dell'espressione.
act: è un oggetto della classe Actions, contenente tutte le variabili con i
    corrispondenti valori che possono essere utilizzati nell'espressione.
L'espressione inizia sempre con un term, quindi:
1) Possiamo direttamente prelevare il term x (chiamando la corrispondente funzione), che c'è sicuramente.
2) Possiamo osservare il token successivo, senza effettivamente andarci, con la funzione peek.
3) Fino a quando il token successivo è un '+' o un '-', possiamo prelevare tale token ed il successivo (che sappiamo 
    essere necessariamente un term y), e svolgere l'effettiva operazione tra i due term x e y.
*/
function expr(tok, act) {
    let x, y, next;
    x = term(tok, act);
    next = tok.peek();
    while (["+", "-"].includes(next)) {
        tok.consume(next);
        y = term(tok, act);
        if (next == "+") {
            x = act.add(x, y);
        } else {
            x = act.sub(x, y);
        }
        next = tok.peek();
    }
    return x;
}

/*
term = factor {( '*' | '/' ) factor}
Un termine è formato sicuramente da un fattore, eventualmente seguito da un numero arbitrario di altri fattori
che vanno a moltiplicarsi o dividersi con il primo. Quindi, se non c'è nulla dopo il primo factor, possiamo 
direttamente ritornare il risultato, in alternativa valutiamo separatamente i due factor e poi facciamo
prodotto/divisione tra i risultati ottenuti. Questo si ripete fino a quando continuiamo a trovare '*' o '/'.
tok: è un oggetto della classe Tokenizer, definito sul testo dell'espressione
    e su una regex per filtrare i diversi token dell'espressione.
act: è un oggetto della classe Actions, contenente tutte le variabili con i
    corrispondenti valori che possono essere utilizzati nell'espressione.
Il term inizia sempre con un factor, quindi:
1) Possiamo direttamente prelevare il factor x (chiamando la corrispondente funzione), che c'è sicuramente.
2) Possiamo osservare il token successivo (se c'è), senza effettivamente andarci, con la funzione peek.
3) Fino a quando il token successivo è '*' o '/', possiamo prelevare tale token ed il successivo (che sappiamo 
    essere necessariamente un factor y), e svolgere l'operazione di moltiplicazione o divisione tra i due factor.
*/
function term(tok, act) {
    let x, y, next;
    x = factor(tok, act);
    next = tok.peek();
    while (["*", "/"].includes(next)) {
        tok.consume(next);
        y = factor(tok, act);
        if (next == "*") {
            x = act.mul(x, y);
        } else {
            x = act.div(x, y);
        }
        next = tok.peek();
    }
    return x;
}

/*
factor = '-' factor | '(' expr ')' | var | num
Un factor è l'elemento base dell'espressione, ossia una costante, una variabile, oppure una sub-espressione.
In questo caso occorre vedere chi è il token successivo (senza effettivamente andarci, i.e. funzione peek).
1) Se questo è un '-', allora sicuramente il token successivo è un factor x, per cui possiamo andare a valutarlo 
    con la stessa funzione factor e svolgere l'operazione corrispondente (i.e. calcolo -x), ritornando il risultato.
2) Se questo è un '(', allora sappiamo che a seguito ci sarà necessariamente una espressione, per cui possiamo 
    andare a valutarla con la funzione expr. In seguito dovremo assicurarci che il successivo token sia un ')',
    per assicurarci che non ci sia un errore di sintassi. Dopo di che, possiamo ritornare il risultato.
3) Se è un valore alfanumerico, corrisponde al nome di una variabile, quindi possiamo ritornarne il valore 
    numerico con il metodo var di act.
4) Il caso rimanente è quello in cui si tratti di una costante, quindi possiamo restituirne il valore effettivo
    con il metodo num di act.
*/
function factor(tok, act) {
    let x, next;
    next = tok.peek();
    if (next == "-") {
        tok.consume("-");
        x = factor(tok, act);
        x = act.opp(x);
    } else if (next == "(") {
        tok.consume("(");
        x = expr(tok, act);
        tok.consume(")");
    } else if (next.search(/^[a-zA-Z0-9]+$/) !== -1) {
        tok.consume(next);
        x = act.var(next);
    } else {
        tok.consume(next);
        x = act.num(next);
    }
    return x;
}


class Actions {
    //this.values contiene le variabili con i loro valori.
    constructor(values) {
        this.values = values;
    }
    //Dato x + y, restituisce + x y
    add(x, y) {
        return `+ ${x} ${y}`;
    }
    //Dato x - y, restituisce - x y
    sub(x, y) {
        return `- ${x} ${y}`;
    }
    //Dato x * y, restituisce * x y
    mul(x, y) {
        return `* ${x} ${y}`;
    }
    //Dato x / y, restituisce / x y
    div(x, y) {
        return `/ ${x} ${y}`;
    }
    //Dato -x, restituisce ~x
    opp(x) {
        return `~${x}`;
    }
    //Restituisce la stessa costante x come stringa
    num(x) {
        return x;
    }
    //Restituisce il nome stesso della variabile x
    var(x) {
        return x;
    }
};


/*
La classe Tokenizer rappresenta un oggetto che consente di trasformare il testo dell'espressione in un insieme di token,
i quali rappresentano i diversi blocchi dell'espressione che dovremo andare a valutare.
Il costruttore prende due parametri: text (il testo dell'espressione) e regex (che individua un singolo token nel testo),
e inizializza una variabile point, la quale serve ad indicare il punto in cui siamo arrivati nel tokenizzare l'espressione.
*/
class Tokenizer {
    constructor(text, regex) {
        this.text = text.trimEnd();
        this.token_re = new RegExp('\s*([A-Za-z0-9\.]+|.?)', 'g');
        this.point = 0;
    }

    /*
    La funzione peek cerca e restituisce il primo token in text (null se non trova nulla), a partire dal
    punto in cui ci troviamo attualmente (ossia il valore della variabile point), senza aggiornare la
    posizione (è come se spiassimo cosa c'è dopo, senza effettivamente andarci).
    */
    peek() {
        let next;
        let lastIndex = this.token_re.lastIndex;
        do {
            next = this.token_re.exec(this.text);
        } while ((next !== null) && (next[0] === " "));
        this.token_re.lastIndex = lastIndex;
        return String(next[0]);
    }

    /*
    La funzione consume funziona come peek, ma aggiorna la posizione (point viene portata al valore del match successivo)
    e non ritorna nulla (va quindi chiamata solamente dopo la funzione peek).
    Il parametro x rappresenza quello che ci aspettiamo di trovare come token (es. se con il peek troviamo un '-', ci 
    aspettiamo di trovare esattamente '-' con il consume). Qualora il match trovato con il consume non sia uguale ad x,
    si genera un errore, altrimenti si aggiorna la variabile point.
    */
    consume(x) {
        let next;
        do {
            next = this.token_re.exec(this.text);
        } while ((next !== null) && (next[0] === " "));
        if (next[0] !== x) {
            try {
                throw new SyntaxError(`Expected ${x}!`);
            } catch (e) {
                console.log(e);
            }
        }
        this.point = this.token_re.lastIndex;
        return next[0];
    }

    /*
    La funzione end serve a capire, dopo aver valutato l'intera espressione, se essa è stata formulata o meno in modo corretto.
    La chiamiamo dopo l'uscita dalla funzione expr, poichè end confronta il valore della variabile point, con la lunghezza del 
    testo rappresentante l'espressione (se essi non coincidono, vuol dire che ci sono ulteriori simboli non riconosciuti 
    nell'espressione, altrimenti va tutto bene e possiamo tenerci il risultato dell'expr).
    */
    end() {
        if (this.point < this.text.length) {
            try {
                throw new SyntaxError("Extra stuff after expression!");
            } catch (e) {
                console.log(e);
            }
        }
    }
};


/*
Semplice funzione di utilità, che:
1) crea la regex per filtrare i token nell'espressione;
2) crea l'oggetto tokenizer con il testo dell'espressione (passato come parametro) e la regex;
3) elabora l'espressione con la funzione expr e memorizza il risultato;
4) se non ci sono errori (li verifica il metodo end del tokenizer), ritorna il risultato corretto.
*/
function parseExpr(text, act) {
    regex = '\s*([A-Za-z0-9\.]+|.?)';
    tok = new Tokenizer(text, regex);
    result = expr(tok, act);
    tok.end();
    return result;
}


function checkCorrect(actual, expected) {
    return actual === expected ? "Correct result." : "Wrong result."; 
}


let act = new Actions();
//Espressioni per testare, nella forma espressione:risultato_atteso.
let expressions = {
    '(((1.5)))': '1.5',
    'w * -z': '* w ~z',
    'x / z * -y': '* / x z ~y',
    'x / 0.5 * --y': '* / x 0.5 ~~y',
    'w': 'w',
    '(x + w) * (x + y) * (y - z)': '* * + x w + x y - y z'
};


for (const expression in expressions) {
    console.log(checkCorrect(parseExpr(expression, act), expressions[expression]));
}


