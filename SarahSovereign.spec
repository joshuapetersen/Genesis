# -*- mode: python ; coding: utf-8 -*-


a = Analysis(
    ['sarah_native.py'],
    pathex=[],
    binaries=[],
    datas=[('AetherCodeStudio/dist', 'AetherCodeStudio/dist')],
    hiddenimports=['uvicorn.logging', 'uvicorn.loops', 'uvicorn.loops.auto', 'uvicorn.protocols', 'uvicorn.protocols.http', 'uvicorn.protocols.http.auto', 'uvicorn.lifespan', 'uvicorn.lifespan.on', 'engineio.async_drivers.threading', 'selenium', 'webdriver_manager', 'pyautogui', 'pywinauto', 'pyscreeze', 'pygetwindow', 'mouseinfo', 'pytweening', 'pyrect', 'pyperclip'],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=['pysqlite2', 'MySQLdb', 'psycopg2'],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='SarahSovereign',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
