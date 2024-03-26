# AMnezia COnfig FIXER

## RU

С переходом на новую версию амнезии потерялась совместимость с конфигами старых версий. Зачастую для того чтобы вернуть совместимость достаточно маленьких преобразований файла

Но так как всё сложилось, что разработчики Amnezia не предоставили возможность импорта или редактирования отдельного конфига, а разработчики QT давно отринулись от бога и создают велосипеды на подобии qComporess.

Так что я просто сделал этот мир чуть лучше и сделать программку, которая фиксит эти конфиги. Это работает не в ста процентах случаях и происхождение многих ошибок мне не понятно. 

### Принцип работы

Программа

1. Считывает конфиг
1. переводит его в BASE64-URL
1. УДАЛЯЕТ первые 4 байта (Лол)
1. Используя zlib распаковывает файлик
1. Находит нужное значение, правит

И далее все в обратную сторону.

### Использование

Если у вас есть раст, то клонируем репозиторий и

> cargo build release

Если нет, то в папке релиз поищи под свою систему

После этого в терминале

> ./amcofixer **конфиг**

И добавляешь new_**конфиг** в Amnezia

## EN

With the transition to a new version of amnesia, compatibility with the configs of older versions was lost. Often, small file conversions are enough to restore compatibility.

But since everything turned out that the Amnezia developers did not provide the ability to import or edit a separate config, and the QT developers have long ago abandoned God and are creating bicycles like qComporess.

So I just made this world a little better and made a program that fixes these configs. This does not work in one hundred percent of cases and the origin of many errors is not clear to me.

### How it's work

Program

1. Reads the config
1. translates it to BASE64-URL
1. DELETES the first 4 bytes (Lol)
1. Using zlib unpacks the file
1. Finds the desired value, edits

And then everything goes in the opposite direction.

### Usage

If you have a rast, then clone the repository and

> cargo build release

If not, then look in the release folder for your system

After that in the terminal

> ./amcofixer **config**

And add new_**config** to Amnezia
