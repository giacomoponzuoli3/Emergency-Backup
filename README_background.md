# Group35
L'applicazione fornisce un modo utile per evitare la perdita di dati a causa di un monitor non funzionante, mentre sei ancora in grado di muovere il mouse. 
Quando si verifica una situazione critica (o se vuoi semplicemente eseguire il backup di alcuni dati), puoi disegnare un semplice gesto con il mouse, 
tracciando, ad esempio, un rettangolo seguendo i bordi del monitor, e quindi confermare con uno scorrimento da sinistra a destra.


## Installazione
L'app è portatile, cioè è un programma software che può essere eseguito su un computer senza dover essere installato nel sistema operativo, quindi è sufficiente utilizzare l'eseguibile appropriato, come elencato di seguito, in base al sistema operativo in uso.

## Funzionamento
Per lanciare i comandi seguenti assicurati di essere nella directory principale del progetto.
Il programma è composto dai seguenti file:
- `mainBackground.rs`: contiene un'applicazione che esegue periodicamente alcune operazioni in background, tra cui la registrazione su un file di log ogni secondo per un periodo di tempo, oltre a configurare l'avvio automatico dell'applicazione all'avvio del sistema operativo, con specifiche implementazioni per Windows, Linux e macOS
- `uninstallBackground.rs`: questo file Rust ha l'obiettivo di disabilitare l'avvio automatico di un'applicazione chiamata "Group-35" e di terminare il processo dell'applicazione, a seconda del sistema operativo in uso (Windows, macOS, o Linux)
### Windows
- **Esecuzione**: lanciare nel terminale il seguente comando
  ``` 
  release\windows\Group35.exe
  ```
- Per qualsiasi **modifica** al codice, disinstallare il programma, quindi eseguire lo script per ricostruire il progetto eseguendo il seguente comando
  ```
  windows_build_release.bat
  ``` 
- **Terminazione processi background**: per terminare l'esecuzione dei vari processi in background eseguire il comando
  ``` 
  release/windows/uninstall.exe
  ```
Lo script `windows_build_release.bat` effettua i seguenti passaggi:
1. Rimozione della cartella `release\windows`
2. Compilazione del progetto
3. Creazione della cartella `release\windows`
4. Copia dell'eseguibile `Group35.exe`
5. Messaggio finale

### MacOS
- **Requisiti**: `osascript` dovrebbe essere installato di default dal Sistema Operativo, se non funziona, verificare l'installazione
- **Esecuzione**: basta lanciare il seguente comando 
  ```
  release/macos/Group35
  ```
  In caso di problemi lanciare singolarmente i vari comandi che permettono di eseguire i vari processi singolarmente:
    - elenco vari programmi (esempio quello della gui ecc)
    - lanciare `release/macos/uninstall`
    - lanciare `release/macos/Group35`
- Per qualsiasi **modifica** del codice, disintallare il programma, ed eseguire lo script per fare il rebuild del progetto 
  ```
  macos_build_release.sh
  ```
  Se hai problemi, assicurati che i permessi di esecuzione siano impostati correttamente. Puoi fare ciò eseguendo:
  ```
  chmod +x release/macos/Group35
  macos_build_release.sh
  ```
- **Terminazione processi background**: per terminare l'esecuzione dei vari processi in background eseguire il comando 
  ```
  release/macos/uninstall
  ```

Lo script `macos_build_release.sh` effettua i seguenti passaggi:
1. Pulisce la cartella `release/macos` per evitare conflitti con build precedenti
2. Compila il progetto Rust in modalità release usando Cargo
3. Crea una directory di destinazione (`release/macos`) e copia lì tutti gli eseguibili e gli asset necessari dalla cartella `target/release`
4. Fornisce feedback all'utente al termine del processo

### Linux
- **Requisito**: eventuali requisiti
- **Esecuzione**: eseguire il seguente comando 
  ```
  release/linux/Group35
  ```
-  Per qualsiasi **modifica** del codice, disintallare il programma, ed eseguire lo script per fare il rebuild del progetto
  ```
  linux_build_release.sh
  ```
  Se hai problemi, assicurati che i permessi di esecuzione siano impostati correttamente. Puoi fare ciò eseguendo:
  ```
  chmod +x release/linux/Group35
  linux_build_release.sh
  ```
- **Terminazione processi background**: per terminare l'esecuzione dei vari processi in background eseguire il comando 
  ```
  release/linux/uninstall
  ```

Lo script `linux_build_release.sh` effettua i seguenti passaggi:

1. Elimina la cartella di rilascio esistente (`release/linux`)
2. Compila il progetto Rust in modalità ottimizzata (`release`)
3. Crea una nuova cartella `release/linux` se non esiste
4. Copia l'eseguibile generato (`Group35`) nella cartella di rilascio
5. Stampa un messaggio per confermare il completamento del processo
