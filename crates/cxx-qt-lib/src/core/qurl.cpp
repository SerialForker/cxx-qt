// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qurl.h"

#include <cxx-qt-lib/assertion_utils.h>

// QUrl has a single pointer as it's member
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v5.15.6-lts-lgpl#n367
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/io/qurl.h?h=v6.2.4#n294
assert_alignment_and_size(QUrl, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QUrl>::value);
static_assert(!::std::is_trivially_copy_constructible<QUrl>::value);

static_assert(!::std::is_trivially_destructible<QUrl>::value);

static_assert(QTypeInfo<QUrl>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QString
qurlAuthority(const QUrl& url)
{
  return url.authority();
}

QString
qurlFileName(const QUrl& url)
{
  return url.fileName();
}

QString
qurlFragment(const QUrl& url)
{
  return url.fragment();
}

QUrl
qurlFromEncoded(const QByteArray& input)
{
  return QUrl::fromEncoded(input);
}

QUrl
qurlFromUserInput(const QString& userInput, const QString& workingDirectory)
{
  return QUrl::fromUserInput(userInput, workingDirectory);
}

QString
qurlHost(const QUrl& url)
{
  return url.host();
}

QString
qurlPath(const QUrl& url)
{
  return url.path();
}

QString
qurlPassword(const QUrl& url)
{
  return url.password();
}

QString
qurlQuery(const QUrl& url)
{
  return url.query();
}

void
qurlSetAuthority(QUrl& url, const QString& authority)
{
  url.setAuthority(authority);
}

void
qurlSetFragment(QUrl& url, const QString& fragment)
{
  url.setFragment(fragment);
}

void
qurlSetHost(QUrl& url, const QString& host)
{
  url.setHost(host);
}

void
qurlSetPassword(QUrl& url, const QString& password)
{
  url.setPassword(password);
}

void
qurlSetPath(QUrl& url, const QString& path)
{
  url.setPath(path);
}

void
qurlSetQuery(QUrl& url, const QString& query)
{
  url.setQuery(query);
}

void
qurlSetScheme(QUrl& url, const QString& scheme)
{
  url.setScheme(scheme);
}

void
qurlSetUrl(QUrl& url, const QString& newUrl)
{
  url.setUrl(newUrl);
}

void
qurlSetUserInfo(QUrl& url, const QString& userInfo)
{
  url.setUserInfo(userInfo);
}

void
qurlSetUserName(QUrl& url, const QString& userName)
{
  url.setUserName(userName);
}

QString
qurlToDisplayString(const QUrl& url)
{
  return url.toDisplayString();
}

QByteArray
qurlToEncoded(const QUrl& url)
{
  return url.toEncoded();
}

QString
qurlToQString(const QUrl& url)
{
  return url.toString();
}

QString
qurlUserInfo(const QUrl& url)
{
  return url.userInfo();
}

QString
qurlUserName(const QUrl& url)
{
  return url.userName();
}

}
}
