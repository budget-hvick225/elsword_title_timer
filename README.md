- Added Grove Timer
- Updated Debug Section to account for new additions
- Every Timer type is now togglable

Make this update description slightly more professional 

This is the project desscription overall

# Elsword Title Timer

**Elsword Title Timer** is a utility that helps you track your titles efficiently in **Elsword**. It’s especially useful in raids, where optimizing your DPS by timing your title effects correctly can make a big difference.

## Safety / Disclaimer

Elsword Title Timer is completely safe. It does **not** modify the game or read its memory. It only listens to your key presses at the operating system level. You could even run it while using Notepad.

Using this tool will **not get you banned**, as it does not interact with the game client in any way.  

**Important:** Even though this utility is safe and not a cheat, it’s best **not to discuss external tools inside the game**, as other players or automated systems may misunderstand.

## Setup

1. Build the app for your platform (Windows/Linux).  
2. Copy your `config.json` file into the same folder where the application was built.
3. You can edit `config.json` manually with a text editor or modify settings directly in the app under the **Settings** tab.

> **Tip:** For best results, make sure your key settings in `config.json` match your in-game key configuration.

## ⚠️ Windows Administrator Requirement

If Elsword is running as Administrator, you must also run **Elsword Title Timer** as Administrator.

Windows does not allow non-elevated applications to listen to key inputs sent to elevated (admin) applications.

If key presses are not being detected while the game is open, try launching the timer with:
→ Right Click → Run as Administrator

This is a Windows permission requirement and is not related to game interaction or memory access.

## Title Processing Instructions

### TSS & FS Titles
1. Press your **Select Title** key.  
2. Press your **TSS / FS** key.  
3. Press your **Awakening / Onion** key.

### NP Title
1. Press your **Select Title** key.  
2. Press your **NP** key.  
3. Press any of your **Skill** keys.
   
### Flow (Atma’Ram Flow Accessories)
- Press any of your **Consumable** keys.

### Lithia Gemstones & Grove (Atma’Ram Grove Accessories)
- Press your **Awakening / Onion** key.

## Quick Reference

| Title / Effect                              | Key Sequence                          |
|---------------------------------------------|---------------------------------------|
| **TSS & FS**                                |Select Title → TSS/FS → Awakening/Onion|
| **NP**                                      | Select Title → NP → Any Skill key     |
| **Flow (Atma Flow)**                        | Any Consumable key                    |
| **Lithia Gemstones & Grove (Atma Grove)**   | Awakening key/Onion                   |
