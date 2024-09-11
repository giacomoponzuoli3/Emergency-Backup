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
- `uninstallBackground.rs`: questo file Rust ha l'obiettivo di disabilitare l'avvio automatico di un'applicazione chiamata "Group-35" e di terminare la sua esecuzione, a seconda del sistema operativo in uso (Windows, macOS, o Linux)
- `backup.rs`: questo programma in Rust è progettato per eseguire un backup di una directory su un drive esterno rimovibile, formata dalle seguenti funzioni:
  - `list_external_drives`: 
    - utilizza la libreria `sysinfo` per ottenere un elenco di drive esterni (rimovibili) collegati al sistema
    - la funzione esamina i dischi collegati e, se il disco è rimovibile, ne raccoglie il percorso di montaggio, restituendo un elenco di stringhe rappresentanti i percorsi di questi drive
  - `copy_directory`:
    - prende due percorsi come input: il percorso sorgente (`src`) e il percorso di destinazione (`dst)`
    - utilizza la libreria `walkdir` per esplorare ricorsivamente tutti i file e le directory all'interno della directory sorgente
    - per ogni file o directory, viene replicata la struttura all'interno della destinazione
    - se si tratta di un file, il file viene copiato e la dimensione del file (in byte) viene sommata a un contatore per tracciare la quantità totale di dati copiati
    - la funzione misura anche il tempo impiegato per eseguire il backup e restituisce sia la quantità totale di byte copiati sia la durata dell'operazione
  - `backupExecute.rs` questa funzione avvia il processo di backup:
    - **ottenere la directory corrente**: Viene determinata la directory corrente in cui si trova l'eseguibile o dove è stato lanciato il programma
    - **elencare i drive esterni**: La funzione richiama list_external_drives per ottenere un elenco di drive esterni disponibili
    - **selezionare il drive**: Se ci sono drive esterni disponibili, il programma li elenca e chiede all'utente di selezionarne uno in base all'indice
    - **creare la directory di destinazione**: Dopo che l'utente ha selezionato un drive, il programma crea una directory di backup su quel drive chiamata "backup"
    - **copiare i file**: Utilizza la funzione copy_directory per copiare tutto il contenuto della directory corrente nella directory di backup appena creata
    - **creare un file di report**: Alla fine del processo, viene creato un file di testo backup_report.txt nella directory di backup, che registra il tempo impiegato e la quantità di byte copiati
    - **messaggio di completamento**: Il programma informa l'utente che il backup è stato completato correttamente

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
